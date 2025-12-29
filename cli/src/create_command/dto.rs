use crate::shared::AccountType;
use chrono::Local;
use diesel::{Connection, Insertable, QueryResult, SqliteConnection};

/// Represents a new account to be inserted into the database.
#[derive(Debug, Insertable)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=cobblepot_data_store::schema::account)]
struct NewAccount {
    pub name: String,
    pub description: Option<String>,
    pub owner: String,
    pub account_type: i32,
    pub opened_on: chrono::NaiveDateTime,
    pub closed_on: Option<chrono::NaiveDateTime>,
}

pub fn create_new_account(
    mut conn: SqliteConnection,
    name: String,
    description: Option<String>,
    owner: Option<String>,
    account_type: Option<AccountType>,
    opened_on: Option<chrono::NaiveDateTime>,
) -> QueryResult<(i32, String)> {
    use cobblepot_data_store::{Account, schema::account};
    use diesel::{RunQueryDsl, dsl::insert_into};
    conn.transaction(|conn| {
        let res = insert_into(account::table)
            .values(NewAccount {
                name,
                description,
                owner: owner.unwrap_or("me".to_string()),
                account_type: account_type.unwrap_or_default().into(),
                opened_on: opened_on.unwrap_or_else(|| Local::now().naive_utc()),
                closed_on: None,
            })
            .get_result::<Account>(conn)?;
        Ok((res.id, res.name))
    })
}

/// Represents a new balance record to be inserted into the database.
#[derive(Debug, Insertable)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=cobblepot_data_store::schema::balance)]
pub struct NewBalance {
    pub memo: String,
    pub amount: f32,
    pub entered_on: chrono::NaiveDateTime,
    pub account_id: i32,
}

pub fn create_new_balance(
    mut conn: SqliteConnection,
    account_id: i32,
    amount: f32,
    memo: Option<String>,
    entered_on: Option<chrono::NaiveDateTime>,
) -> QueryResult<(i32, String)> {
    use cobblepot_data_store::{Balance, schema::balance};
    use diesel::{RunQueryDsl, dsl::insert_into};
    let entered_on = entered_on.unwrap_or_else(|| Local::now().naive_utc());
    conn.transaction(|conn| {
        let res = insert_into(balance::table)
            .values(NewBalance {
                account_id,
                amount,
                entered_on,
                memo: memo.unwrap_or_else(|| {
                    format!(
                        "Account {} current balance of {} entered on {:?}",
                        account_id,
                        amount,
                        entered_on.format("%v").to_string()
                    )
                }),
            })
            .get_result::<Balance>(conn)?;
        Ok((res.id, res.memo))
    })
}
