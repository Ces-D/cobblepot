use diesel::{QueryResult, SqliteConnection};

use crate::apply::model::JSONApplyRecurringTransaction;

pub fn insert_applied_recurring_transaction(
    args: JSONApplyRecurringTransaction,
    connection: SqliteConnection,
) -> QueryResult<()> {
    todo!()
}
