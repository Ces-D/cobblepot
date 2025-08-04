use crate::{
    infrastructure::database::DbPool,
    recurring_transaction::model::{
        InsertableRecurringTransaction, JSONCloseRecurringTransaction,
        JSONListRecurringTransactions, JSONOpenRecurringTransaction, RecurringTransaction,
        RecurringTransactionList,
    },
    schema::recurring_transactions::dsl::{account_id, closed, id, recurring_transactions},
};
use actix_web::{HttpResponse, Scope, web};
use cobblepot_core::error::CobblepotResult;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, insert_into, update};

async fn list_recurring_transactions(
    pool: web::Data<DbPool>,
    filters: web::Query<JSONListRecurringTransactions>,
) -> CobblepotResult<RecurringTransactionList> {
    let args = filters.into_inner();
    let trans: CobblepotResult<Vec<RecurringTransaction>> = web::block(move || {
        let mut conn = pool.get()?;
        let mut query = recurring_transactions.into_boxed();

        if let Some(acc_id) = args.account_id {
            query = query.filter(account_id.eq(acc_id));
        }

        let res: Vec<RecurringTransaction> = query
            .offset(args.offset.unwrap_or(0))
            .limit(args.limit.unwrap_or(25))
            .load(&mut conn)?;
        Ok(res)
    })
    .await?;

    Ok(RecurringTransactionList(trans?))
}

async fn insert_new_recurring_transaction(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONOpenRecurringTransaction>,
) -> CobblepotResult<RecurringTransaction> {
    let args = payload.into_inner();

    web::block(move || {
        let mut conn = pool.get()?;
        let insertable: InsertableRecurringTransaction = args.try_into()?;
        let res = insert_into(recurring_transactions)
            .values(insertable)
            .get_result::<RecurringTransaction>(&mut conn)?;
        Ok(res)
    })
    .await?
}

async fn close_recurring_transaction(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONCloseRecurringTransaction>,
) -> CobblepotResult<HttpResponse> {
    let args = payload.into_inner();
    let transaction: CobblepotResult<usize> = web::block(move || {
        let mut conn = pool.get()?;
        let res = update(recurring_transactions.filter(id.eq(args.id)))
            .set(closed.eq(true))
            .execute(&mut conn)?;
        Ok(res)
    })
    .await?;
    if transaction.is_ok() {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::InternalServerError().finish())
    }
}

pub fn recurring_transaction_scope() -> Scope {
    web::scope("/recurring_transaction")
        .route("/list", web::get().to(list_recurring_transactions))
        .route("/open", web::post().to(insert_new_recurring_transaction))
        .route("/close", web::delete().to(close_recurring_transaction))
}

#[cfg(test)]
mod test {
    use actix_web::{App, test, web};

    use crate::{
        account::model::Account,
        recurring_transaction::model::{
            JSONCloseRecurringTransaction, RecurringTransaction,
            test_utils::create_dummy_open_recurring_transaction,
        },
        test_utils::create_dummy_open_account,
    };

    #[actix_web::test]
    async fn recurring_transaction_lifecycle_successful() {
        let database_pool = crate::infrastructure::database::database_memory_pool().unwrap();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(database_pool))
                .service(super::recurring_transaction_scope())
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

        let create_recurring_req = test::TestRequest::with_uri("/recurring_transaction/open")
            .method(actix_web::http::Method::POST)
            .set_json(create_dummy_open_recurring_transaction(created_account.id))
            .to_request();
        let recurring_res = test::call_service(&app, create_recurring_req).await;
        assert!(recurring_res.status().is_success());
        let created_recurring_transaction: RecurringTransaction =
            test::read_body_json(recurring_res).await;

        let close_recurring_req = test::TestRequest::with_uri("/recurring_transaction/close")
            .method(actix_web::http::Method::DELETE)
            .set_json(JSONCloseRecurringTransaction {
                id: created_recurring_transaction.id,
            })
            .to_request();
        let close_res = test::call_service(&app, close_recurring_req).await;
        assert!(close_res.status().is_success());
    }
}
