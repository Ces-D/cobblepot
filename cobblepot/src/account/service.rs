use crate::{
    account::model::{
        Account, InsertableAccount, JSONCloseAccount, JSONOpenAccount, JSONUpdateAccount,
        UpdatableAccount,
    },
    schema::account::dsl::{account, closed_on, id},
    shared::DATETIME_FORMAT,
};
use chrono::Utc;
use diesel::{
    Connection, ExpressionMethods, QueryResult, RunQueryDsl, insert_into,
    query_dsl::methods::FilterDsl, update,
};

pub fn insert_new_account(
    args: JSONOpenAccount,
    mut connection: diesel::SqliteConnection,
) -> QueryResult<Account> {
    connection.transaction(|conn| {
        let insertable: InsertableAccount = args.into();
        let res = insert_into(account).values(insertable).get_result::<Account>(conn)?;
        Ok(res)
    })
}

pub fn update_account(
    args: JSONUpdateAccount,
    mut connection: diesel::SqliteConnection,
) -> QueryResult<Account> {
    connection.transaction(|conn| {
        let updateable: UpdatableAccount = args.into();
        let res = update(account).set(updateable).get_result::<Account>(conn)?;
        Ok(res)
    })
}

pub fn close_account(
    args: JSONCloseAccount,
    mut connection: diesel::SqliteConnection,
) -> QueryResult<usize> {
    connection.transaction(|conn| {
        let res = update(account.filter(id.eq(args.id)))
            .set(
                closed_on.eq(args
                    .closed_on
                    .unwrap_or(Utc::now())
                    .format(DATETIME_FORMAT)
                    .to_string()),
            )
            .execute(conn)?;
        Ok(res)
    })
}
