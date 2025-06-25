use crate::{
    balance::model::{
        Balance, InsertableBalance, JSONOpenBalance, JSONUpdateBalance, UpdatableBalance,
    },
    infrastructure::database::DbPool,
    schema::balance::dsl::{balance, id},
    shared::CobblepotResult,
};
use actix_web::{Scope, web};
use diesel::{ExpressionMethods, RunQueryDsl, insert_into, query_dsl::methods::FilterDsl, update};

async fn insert_new_balance(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONOpenBalance>,
) -> CobblepotResult<Balance> {
    let args = payload.into_inner();
    let bal = web::block(move || {
        let mut conn = pool.get().unwrap();
        let insertable: InsertableBalance = args.into();
        let res = insert_into(balance).values(insertable).get_result::<Balance>(&mut conn)?;
        Ok(res)
    })
    .await?;
    bal
}

async fn update_balance(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONUpdateBalance>,
) -> CobblepotResult<Balance> {
    let args = payload.into_inner();
    let balance_id = args.id;
    let bal = web::block(move || {
        let mut conn = pool.get().unwrap();
        let updatable: UpdatableBalance = args.into();
        let res = update(balance.filter(id.eq(balance_id)))
            .set(updatable)
            .get_result::<Balance>(&mut conn)?;
        Ok(res)
    })
    .await?;
    bal
}

pub fn balance_scope() -> Scope {
    web::scope("/balance")
        .route("/open", web::post().to(insert_new_balance))
        .route("/update", web::put().to(update_balance))
}

#[cfg(test)]
mod test {
    use crate::{
        account::model::{Account, test_utils::create_dummy_open_account},
        balance::model::{
            Balance,
            test_utils::{create_dummy_open_balance, create_dummy_update_balance},
        },
    };
    use actix_web::{App, test, web};

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
            .set_json(create_dummy_open_account())
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
        println!("{}", balance_res.status());
        assert!(balance_res.status().is_success());
        let updated_balance: Balance = test::read_body_json(balance_res).await;
        assert_eq!(updated_balance.amount, 200.0, "The updated amount should be 200.0");
        assert_eq!(
            updated_balance.id, balance_id,
            "The updated balance should have the same id as the created balance"
        );
    }
}
