use crate::shared::AccountType;
use chrono::{Months, Utc};
use cobblepot_data_store::{Account, Balance, Budget, BudgetItem, BudgetRecurrence};
use diesel::{Connection, QueryResult, SqliteConnection};

pub fn get_filtered_accounts(
    mut conn: SqliteConnection,
    account_type: Option<AccountType>,
    opened_by: Option<chrono::NaiveDateTime>,
) -> QueryResult<Vec<Account>> {
    use cobblepot_data_store::schema::account;
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
    let opened_by = cobblepot_data_store::UnixTimestamp(opened_by.unwrap_or_else(|| {
        Utc::now().checked_sub_months(Months::new(12 * 10)).unwrap().naive_utc()
    }));
    match account_type {
        Some(act) => {
            let act: i32 = act.into();
            conn.transaction(|conn| {
                let res = account::table
                    .filter(account::account_type.eq(act))
                    .filter(account::opened_on.ge(opened_by))
                    .select(Account::as_select())
                    .load(conn)?;
                Ok(res)
            })
        }
        None => conn.transaction(|conn| {
            let res = account::table
                .filter(account::opened_on.ge(opened_by))
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
    let entered_by = cobblepot_data_store::UnixTimestamp(entered_by.unwrap_or_else(|| {
        Utc::now().checked_sub_months(Months::new(12 * 10)).unwrap().naive_utc()
    }));
    match account_id {
        Some(account_id) => conn.transaction(|conn| {
            let res = balance::table
                .filter(balance::account_id.eq(account_id))
                .filter(balance::entered_on.ge(entered_by))
                .select(Balance::as_select())
                .load(conn)?;

            Ok(res)
        }),
        None => conn.transaction(|conn| {
            let res = balance::table
                .filter(balance::entered_on.ge(entered_by))
                .select(Balance::as_select())
                .load(conn)?;
            Ok(res)
        }),
    }
}

pub fn get_filtered_budgets(
    mut conn: SqliteConnection,
) -> QueryResult<Vec<(Budget, Option<BudgetRecurrence>)>> {
    // TODO: add a flag for filtering based on closed -> Use the UNTIL on rrule to populate a dt_end
    use cobblepot_data_store::schema::{budget, budget_recurrence};
    use diesel::{
        ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl, RunQueryDsl,
        SelectableHelper,
    };
    conn.transaction(|conn| {
        let res = budget::table
            .left_join(
                budget_recurrence::table
                    .on(budget::budget_recurrence_id.eq(budget_recurrence::id.nullable())),
            )
            .select((Budget::as_select(), Option::<BudgetRecurrence>::as_select()))
            .load(conn)?;
        Ok(res)
    })
}

pub fn get_filtered_budget_items(
    mut conn: SqliteConnection,
    budget_id: i32,
) -> QueryResult<Vec<(BudgetItem, Option<BudgetRecurrence>)>> {
    use cobblepot_data_store::schema::{budget_item, budget_recurrence};
    use diesel::{
        ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl, RunQueryDsl,
        SelectableHelper, query_dsl::methods::FilterDsl,
    };
    conn.transaction(|conn| {
        let res = FilterDsl::filter(budget_item::table, budget_item::budget_id.eq(budget_id))
            .left_join(
                budget_recurrence::table
                    .on(budget_item::budget_recurrence_id.eq(budget_recurrence::id.nullable())),
            )
            .select((BudgetItem::as_select(), Option::<BudgetRecurrence>::as_select()))
            .load(conn)?;
        Ok(res)
    })
}
