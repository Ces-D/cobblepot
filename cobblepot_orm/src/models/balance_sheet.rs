use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Insertable)]
#[diesel(table_name = crate::schema::balance)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(id))]
pub struct Balance {
    id: i32,
    account_code: i32,
    current_balance: i32,
    journal_entry: i32,
}
