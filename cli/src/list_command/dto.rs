use crate::shared::AccountType;
use chrono::{Months, Utc};
use cobblepot_data_store::{Account, Balance};
use diesel::{Connection, QueryResult, SqliteConnection};

pub fn get_filtered_accounts(
    mut conn: SqliteConnection,
    account_type: Option<AccountType>,
    opened_by: Option<chrono::NaiveDateTime>,
) -> QueryResult<Vec<Account>> {
    use cobblepot_data_store::schema::account;
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
    let opened_by = opened_by.unwrap_or_else(|| {
        Utc::now().checked_sub_months(Months::new(12 * 10)).unwrap().naive_utc()
    });
    match account_type {
        Some(act) => {
            let act: i32 = act.into();
            conn.transaction(|conn| {
                let res = account::table
                    .filter(account::account_type.eq(act))
                    .filter(account::opened_on.gt(opened_by))
                    .select(Account::as_select())
                    .load(conn)?;
                Ok(res)
            })
        }
        None => conn.transaction(|conn| {
            let res = account::table
                .filter(account::opened_on.gt(opened_by))
                .select(Account::as_select())
                .load(conn)?;
            Ok(res)
        }),
    }
}

pub fn get_filtered_balances(
    mut conn: SqliteConnection,
    account_id: Option<i32>,
    entered_by: Option<chrono::NaiveDateTime>,
) -> QueryResult<Vec<Balance>> {
    use cobblepot_data_store::schema::balance;
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
    let entered_by = entered_by.unwrap_or_else(|| {
        Utc::now().checked_sub_months(Months::new(12 * 10)).unwrap().naive_utc()
    });
    match account_id {
        Some(account_id) => conn.transaction(|conn| {
            let res = balance::table
                .filter(balance::account_id.eq(account_id))
                .filter(balance::entered_on.gt(entered_by))
                .select(Balance::as_select())
                .load(conn)?;

            Ok(res)
        }),
        None => conn.transaction(|conn| {
            let res = balance::table
                .filter(balance::entered_on.gt(entered_by))
                .select(Balance::as_select())
                .load(conn)?;
            Ok(res)
        }),
    }
}
