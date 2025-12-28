use crate::shared::AccountType;
use chrono::{Months, Utc};
use cobblepot_data_store::Account;
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
