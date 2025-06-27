use crate::{
    account::model::{
        Account, ClosableAccount, InsertableAccount, JSONCloseAccount, JSONOpenAccount,
        JSONUpdateAccount, UpdatableAccount,
    },
    infrastructure::database::DbPool,
    schema::account::dsl::{account, id},
    shared::CobblepotResult,
};
use actix_web::{HttpResponse, Scope, web};
use diesel::{ExpressionMethods, RunQueryDsl, insert_into, query_dsl::methods::FilterDsl, update};

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
        .route("/open", web::post().to(insert_new_account))
        .route("/update", web::put().to(update_account))
        .route("/close", web::delete().to(close_account))
}

#[cfg(test)]
mod test {
    use crate::account::model::{
        Account, JSONCloseAccount,
        test_utils::{create_dummy_open_account, create_dummy_update_account},
    };
    use actix_web::{App, test, web};

    #[actix_web::test]
    async fn account_lifecycle_successful() {
        let database_pool = crate::infrastructure::database::database_memory_pool().unwrap();
        let app = test::init_service(
            App::new().app_data(web::Data::new(database_pool)).service(super::account_scope()),
        )
        .await;

        let asset_res = test::TestRequest::with_uri("/account/open")
            .method(actix_web::http::Method::POST)
            .set_json(create_dummy_open_account())
            .send_request(&app)
            .await;

        println!("{:?}", asset_res.response().body());
        assert!(asset_res.status().is_success());

        let created: Account = test::read_body_json(asset_res).await;
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
