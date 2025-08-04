use crate::{
    account::model::{
        Account, AccountList, ClosableAccount, InsertableAccount, JSONCloseAccount,
        JSONListAccounts, JSONOpenAccount, JSONUpdateAccount, UpdatableAccount,
    },
    infrastructure::database::DbPool,
    schema::account::dsl::{account, account_type, closed_on, id, opened_on},
};
use cobblepot_core::error::CobblepotResult;

use actix_web::{HttpResponse, Scope, web};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, insert_into, update};

async fn list_accounts(
    pool: web::Data<DbPool>,
    filters: web::Query<JSONListAccounts>,
) -> CobblepotResult<AccountList> {
    let args = filters.into_inner();
    let accs: CobblepotResult<Vec<Account>> = web::block(move || {
        let mut conn = pool.get().unwrap();
        let mut query = account.into_boxed();

        if let Some(acc_type) = args.account_type {
            query = query.filter(account_type.eq(acc_type as i32));
        }
        if let Some(opened_after) = args.opened_after {
            query = query.filter(opened_on.gt(opened_after.naive_utc()));
        }
        if let Some(closed_after) = args.closed_after {
            query = query.filter(closed_on.gt(closed_after.naive_utc()));
        }
        let res: Vec<Account> = query
            .offset(args.offset.unwrap_or(0))
            .limit(args.limit.unwrap_or(25))
            .load(&mut conn)?;
        Ok(res)
    })
    .await?;

    Ok(AccountList(accs?))
}

async fn insert_new_account(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONOpenAccount>,
) -> CobblepotResult<Account> {
    let args = payload.into_inner();
    let acc = web::block(move || {
        let mut conn = pool.get().unwrap();
        let insertable: InsertableAccount = args.into();
        let res = insert_into(account).values(insertable).get_result::<Account>(&mut conn)?;
        Ok(res)
    })
    .await?;
    acc
}

async fn update_account(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONUpdateAccount>,
) -> CobblepotResult<Account> {
    let args = payload.into_inner();
    let account_id = args.id;
    let acc = web::block(move || {
        let mut conn = pool.get().unwrap();
        let updatable: UpdatableAccount = args.into();
        let res = update(account.filter(id.eq(account_id)))
            .set(updatable)
            .get_result::<Account>(&mut conn)?;
        Ok(res)
    })
    .await?;
    acc
}

async fn close_account(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONCloseAccount>,
) -> CobblepotResult<HttpResponse> {
    let args = payload.into_inner();
    let acc = web::block(move || {
        let mut conn = pool.get().unwrap();
        let closable: ClosableAccount = args.into();
        update(account.filter(id.eq(args.id))).set(closable).execute(&mut conn)
    })
    .await?;
    if acc.is_ok() {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::InternalServerError().finish())
    }
}
pub fn account_scope() -> Scope {
    web::scope("/account")
        .route("/list", web::get().to(list_accounts))
        .route("/open", web::post().to(insert_new_account))
        .route("/update", web::put().to(update_account))
        .route("/close", web::delete().to(close_account))
}

#[cfg(test)]
mod test {
    use crate::{
        account::model::{
            Account, JSONCloseAccount, JSONListAccounts, test_utils::create_dummy_update_account,
        },
        shared::AccountType,
        test_utils::{create_dummy_open_account, seed_database},
    };

    use actix_web::{App, test, web};
    use chrono::{Months, Utc};

    #[actix_web::test]
    async fn list_accounts_successful() {
        let database_pool = crate::infrastructure::database::database_memory_pool().unwrap();

        let seed_conn = database_pool.get().unwrap();
        seed_database(seed_conn, Some(AccountType::Liability)).unwrap();

        let app = test::init_service(
            App::new().app_data(web::Data::new(database_pool)).service(super::account_scope()),
        )
        .await;

        let opened_after = Utc::now().checked_sub_months(Months::new(15)).unwrap();
        let filter_query = serde_urlencoded::to_string(JSONListAccounts {
            limit: Some(5),
            offset: Some(1),
            account_type: Some(AccountType::Liability),
            opened_after: Some(opened_after),
            closed_after: None,
        })
        .unwrap();
        let url = format!("/account/list?{}", filter_query);
        let filter_accounts_res = test::TestRequest::with_uri(url.as_str())
            .method(actix_web::http::Method::GET)
            .send_request(&app)
            .await;

        assert!(filter_accounts_res.status().is_success());
        let paginated_accounts: Vec<Account> = test::read_body_json(filter_accounts_res).await;
        assert!(
            paginated_accounts.iter().all(|v| v.opened_on.date() > opened_after.date_naive()),
            "All accounts should be opened within the last 15 months"
        );
        assert!(
            paginated_accounts
                .iter()
                .all(|v| AccountType::from(v.account_type) == AccountType::Liability),
            "All accounts should be of type Liability"
        );
        // Skips the first otherwise it should be 3
        assert_eq!(paginated_accounts.len(), 2);
    }

    #[actix_web::test]
    async fn account_lifecycle_successful() {
        let database_pool = crate::infrastructure::database::database_memory_pool().unwrap();
        let app = test::init_service(
            App::new().app_data(web::Data::new(database_pool)).service(super::account_scope()),
        )
        .await;

        let open_res = test::TestRequest::with_uri("/account/open")
            .method(actix_web::http::Method::POST)
            .set_json(create_dummy_open_account(None))
            .send_request(&app)
            .await;

        assert!(open_res.status().is_success());

        let created: Account = test::read_body_json(open_res).await;
        let original_id = created.id;

        let update_res = test::TestRequest::with_uri("/account/update")
            .method(actix_web::http::Method::PUT)
            .set_json(create_dummy_update_account(&created))
            .send_request(&app)
            .await;

        assert!(update_res.status().is_success());

        let update: Account = test::read_body_json(update_res).await;

        assert_eq!(original_id, update.id, "Updated should have the same id");
        assert_ne!(
            created.description, update.description,
            "Updated should have a different description"
        );
        assert!(
            created.opened_on > update.opened_on,
            "We updated the account to be opened 6 months before utc::now()"
        );
        assert_ne!(
            created.account_type, update.account_type,
            "Updated should have a different account type"
        );

        let close_res = test::TestRequest::with_uri("/account/close")
            .method(actix_web::http::Method::DELETE)
            .set_json(JSONCloseAccount {
                id: original_id,
                closed_on: None,
            })
            .send_request(&app)
            .await;
        assert!(close_res.status().is_success());
    }
}
