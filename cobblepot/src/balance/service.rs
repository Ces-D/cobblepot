use crate::{
    balance::model::{
        Balance, InsertableBalance, JSONOpenBalance, JSONUpdateBalance, UpdatableBalance,
    },
    schema::balance::dsl::balance,
};
use diesel::{Connection, QueryResult, RunQueryDsl, insert_into, update};

pub fn insert_new_balance(
    args: JSONOpenBalance,
    mut connection: diesel::SqliteConnection,
) -> QueryResult<Balance> {
    connection.transaction(|conn| {
        let insertable: InsertableBalance = args.into();
        let res = insert_into(balance).values(insertable).get_result::<Balance>(conn)?;
        Ok(res)
    })
}

pub fn update_balance(
    args: JSONUpdateBalance,
    mut connection: diesel::SqliteConnection,
) -> QueryResult<Balance> {
    connection.transaction(|conn| {
        let updatable: UpdatableBalance = args.into();
        let res = update(balance).set(updatable).get_result::<Balance>(conn)?;
        Ok(res)
    })
}
