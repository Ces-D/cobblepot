use crate::recurring_transation::model::{
    CliCloseRecurringTransaction, CliOpenRecurringTransaction, InsertableRecurringTransaction,
    RecurringTransaction,
};
use crate::schema::recurring_transactions::dsl::{id, recurring_transactions, status};
use crate::shared::{CobblepotError, RecurringStatus};
use diesel::{
    Connection, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, insert_into, update,
};

pub fn insert_new_recurring_transaction(
    args: CliOpenRecurringTransaction,
    mut connection: diesel::SqliteConnection,
) -> Result<RecurringTransaction, CobblepotError> {
    connection.transaction(|conn| {
        let insertable: InsertableRecurringTransaction = args.try_into()?;
        let res = insert_into(recurring_transactions)
            .values(insertable)
            .get_result::<RecurringTransaction>(conn)?;
        Ok(res)
    })
}

pub fn close_recurring_transaction(
    args: CliCloseRecurringTransaction,
    mut connection: diesel::SqliteConnection,
) -> QueryResult<usize> {
    connection.transaction(|conn| {
        let res = update(recurring_transactions.filter(id.eq(args.id)))
            .set(status.eq(RecurringStatus::Closed as i32))
            .execute(conn)?;
        Ok(res)
    })
}
