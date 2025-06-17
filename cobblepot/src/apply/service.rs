use actix_web::web;

use crate::{
    apply::model::{AppliedRecurringTransaction, JSONApplyRecurringTransaction},
    infrastructure::database::DbPool,
    shared::CobblepotResult,
};

pub fn insert_applied_recurring_transaction(
    pool: web::Data<DbPool>,
    payload: web::Json<JSONApplyRecurringTransaction>,
) -> CobblepotResult<AppliedRecurringTransaction> {
    todo!()
}
