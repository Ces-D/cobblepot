use diesel::query_dsl::methods::FilterDsl;
use diesel::{insert_into, SqliteConnection};
use diesel::{ExpressionMethods, RunQueryDsl};

use crate::models;
use crate::models::account::Account;

pub fn create_account(
    new_account: models::account::NewAccount,
    connection: &mut SqliteConnection,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::account::dsl::account;
    insert_into(account).values(new_account).execute(connection)
}

pub fn read_account_by_account_code(
    searching_code: i32,
    connection: &mut SqliteConnection,
) -> Result<Account, diesel::result::Error> {
    use crate::schema::account::dsl;
    dsl::account
        .filter(dsl::account_code.eq(searching_code))
        .first::<models::account::Account>(connection)
}
