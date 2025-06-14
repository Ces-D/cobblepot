use crate::{
    recurring_transation::model::{
        InsertableRecurringTransaction, JSONCloseRecurringTransaction,
        JSONOpenRecurringTransaction, RecurringTransaction,
    },
    schema::recurring_transactions::dsl::{closed, id, recurring_transactions},
    shared::CobblepotError,
};
use diesel::{
    Connection, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, insert_into, update,
};

pub fn insert_new_recurring_transaction(
    args: JSONOpenRecurringTransaction,
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
    args: JSONCloseRecurringTransaction,
    mut connection: diesel::SqliteConnection,
) -> QueryResult<usize> {
    connection.transaction(|conn| {
        let res = update(recurring_transactions.filter(id.eq(args.id)))
            .set(closed.eq(true))
            .execute(conn)?;
        Ok(res)
    })
}
