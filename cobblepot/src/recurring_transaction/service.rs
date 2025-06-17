use crate::{
    infrastructure::database::DbPool,
    recurring_transaction::model::{
        InsertableRecurringTransaction, JSONCloseRecurringTransaction,
        JSONOpenRecurringTransaction, RecurringTransaction,
    },
    schema::recurring_transactions::dsl::{closed, id, recurring_transactions},
    shared::CobblepotResult,
};
use actix_web::{HttpResponse, web};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, insert_into, update};

pub async fn insert_new_recurring_transaction(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONOpenRecurringTransaction>,
) -> CobblepotResult<RecurringTransaction> {
    let args = payload.into_inner();
    let transaction = web::block(move || {
        let mut conn = pool.get().unwrap();
        let insertable: InsertableRecurringTransaction = args.try_into()?;
        let res = insert_into(recurring_transactions)
            .values(insertable)
            .get_result::<RecurringTransaction>(&mut conn)?;
        Ok(res)
    })
    .await?;
    transaction
}

pub async fn close_recurring_transaction(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONCloseRecurringTransaction>,
) -> CobblepotResult<HttpResponse> {
    let args = payload.into_inner();
    let transaction = web::block(move || {
        let mut conn = pool.get().unwrap();
        update(recurring_transactions.filter(id.eq(args.id)))
            .set(closed.eq(true))
            .execute(&mut conn)
    })
    .await?;
    if transaction.is_ok() {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::InternalServerError().finish())
    }
}
