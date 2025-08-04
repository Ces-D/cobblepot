use crate::{
    balance::model::{
        Balance, BalanceList, InsertableBalance, JSONListBalances, JSONOpenBalance,
        JSONUpdateBalance, UpdatableBalance,
    },
    infrastructure::database::DbPool,
    schema::balance::dsl::{account_id, balance, entered_on, id},
};
use actix_web::{Scope, web};
use cobblepot_core::error::CobblepotResult;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, insert_into, update};

async fn list_balances(
    pool: web::Data<DbPool>,
    filters: web::Query<JSONListBalances>,
) -> CobblepotResult<BalanceList> {
    let args = filters.into_inner();
    let bals: CobblepotResult<Vec<Balance>> = web::block(move || {
        let mut conn = pool.get()?;
        let mut query = balance.into_boxed();

        if let Some(entered_after) = args.entered_after {
            query = query.filter(entered_on.gt(entered_after.naive_utc()));
        }
        if let Some(acc_id) = args.account_id {
            query = query.filter(account_id.eq(acc_id));
        }
        let res: Vec<Balance> = query
            .offset(args.offset.unwrap_or(0))
            .limit(args.limit.unwrap_or(25))
            .load(&mut conn)?;
        Ok(res)
    })
    .await?;

    Ok(BalanceList(bals?))
}

async fn insert_new_balance(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONOpenBalance>,
) -> CobblepotResult<Balance> {
    let args = payload.into_inner();

    web::block(move || {
        let mut conn = pool.get()?;
        let insertable: InsertableBalance = args.into();
        let res = insert_into(balance).values(insertable).get_result::<Balance>(&mut conn)?;
        Ok(res)
    })
    .await?
}

async fn update_balance(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONUpdateBalance>,
) -> CobblepotResult<Balance> {
    let args = payload.into_inner();
    let balance_id = args.id;

    web::block(move || {
        let mut conn = pool.get()?;
        let updatable: UpdatableBalance = args.into();
        let res = update(balance.filter(id.eq(balance_id)))
            .set(updatable)
            .get_result::<Balance>(&mut conn)?;
        Ok(res)
    })
    .await?
}

pub fn balance_scope() -> Scope {
    web::scope("/balance")
        .route("/list", web::get().to(list_balances))
        .route("/open", web::post().to(insert_new_balance))
        .route("/update", web::put().to(update_balance))
}

#[cfg(test)]
mod test {
    use crate::{
        account::model::Account,
        balance::model::{Balance, JSONListBalances, test_utils::create_dummy_update_balance},
        shared::AccountType,
        test_utils::{create_dummy_open_account, create_dummy_open_balance, seed_database},
    };
    use actix_web::{App, test, web};
    use chrono::{Months, Utc};

    #[actix_web::test]
    async fn list_balances_successful() {
        let database_pool = crate::infrastructure::database::database_memory_pool().unwrap();

        let seed_conn = database_pool.get().unwrap();
        seed_database(seed_conn, Some(AccountType::Liability)).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(database_pool))
                .service(super::balance_scope())
                .service(crate::account::service::account_scope()),
        )
        .await;

        let created_after = Utc::now().checked_sub_months(Months::new(15)).unwrap();
        let filter_query = serde_urlencoded::to_string(JSONListBalances {
            limit: Some(5),
            offset: Some(1),
            entered_after: Some(created_after),
            account_id: Some(1),
        })
        .unwrap();

        let url = format!("/balance/list?{}", filter_query);
        let filter_balance_res = test::TestRequest::with_uri(url.as_str())
            .method(actix_web::http::Method::GET)
            .send_request(&app)
            .await;
        assert!(filter_balance_res.status().is_success());
        let paginated_balances: Vec<Balance> = test::read_body_json(filter_balance_res).await;
        assert!(
            paginated_balances.iter().all(|v| v.entered_on.date() > created_after.date_naive()),
            "All balances should be opened within the last 15 months"
        );
        assert!(
            paginated_balances.iter().all(|v| v.account_id == 1),
            "All accounts should belong to the first account"
        );

        assert_eq!(paginated_balances.len(), 5);
    }

    #[actix_web::test]
    async fn balance_lifecycle_successful() {
        let database_pool = crate::infrastructure::database::database_memory_pool().unwrap();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(database_pool))
                .service(super::balance_scope())
                .service(crate::account::service::account_scope()),
        )
        .await;

        let create_asset_req = test::TestRequest::with_uri("/account/open")
            .method(actix_web::http::Method::POST)
            .set_json(create_dummy_open_account(None))
            .to_request();
        let asset_res = test::call_service(&app, create_asset_req).await;
        assert!(asset_res.status().is_success());
        let created_account: Account = test::read_body_json(asset_res).await;

        let balance_create_req = test::TestRequest::with_uri("/balance/open")
            .method(actix_web::http::Method::POST)
            .set_json(create_dummy_open_balance(created_account.id))
            .to_request();
        let balance_res = test::call_service(&app, balance_create_req).await;
        assert!(balance_res.status().is_success());
        let created_balance: Balance = test::read_body_json(balance_res).await;
        let balance_id = created_balance.id;
        assert_eq!(created_balance.amount, 100.0, "The entered amount should be 100.0");

        let balance_update_req = test::TestRequest::with_uri("/balance/update")
            .method(actix_web::http::Method::PUT)
            .set_json(create_dummy_update_balance(&created_balance))
            .to_request();
        let balance_res = test::call_service(&app, balance_update_req).await;
        assert!(balance_res.status().is_success());
        let updated_balance: Balance = test::read_body_json(balance_res).await;
        assert_eq!(updated_balance.amount, 200.0, "The updated amount should be 200.0");
        assert_eq!(
            updated_balance.id, balance_id,
            "The updated balance should have the same id as the created balance"
        );
    }
}
