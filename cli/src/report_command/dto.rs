use cobblepot_data_store::{RecurrenceRule, UnixTimestamp};
use diesel::{
    Connection, QueryResult, SqliteConnection,
    prelude::QueryableByName,
    sql_types::{Float, Integer, Nullable, Text},
};
use serde::{Deserialize, Serialize};

// Field names must match the column aliases in the SELECT clause.
#[derive(Debug, QueryableByName, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LatestBalanceRow {
    #[diesel(sql_type = Integer)]
    pub account_id: i32,
    #[diesel(sql_type = Integer)]
    pub account_type: i32,
    #[diesel(sql_type = Text)]
    pub account_name: String,
    #[diesel(sql_type = Integer)]
    pub balance_id: i32,
    #[diesel(sql_type = Float)]
    pub amount: f32,
    #[diesel(sql_type = Integer)]
    pub entered_on: UnixTimestamp,
}

// Retrieves the latest balance entry for each open account using raw SQL.
//
// # SQL Query Breakdown
//
// 1. `FROM account a INNER JOIN balance b ON b.account_id = a.id`
//    - Joins the account and balance tables on the foreign key relationship
//    - INNER JOIN excludes accounts that have no balance entries
//
// 2. `WHERE a.closed_on IS NULL`
//    - Filters to only include open accounts (accounts without a closing date)
//
// 3. Correlated Subquery: `b.entered_on = (SELECT MAX(b2.entered_on) ...)`
//    - For each row in the outer query, the subquery finds the maximum
//      `entered_on` timestamp for that specific account
//    - `WHERE b2.account_id = a.id` correlates the subquery to the current
//      account being evaluated in the outer query
//    - Only balance rows where `entered_on` equals that maximum are included,
//      effectively giving us the latest balance per account
//
// # Example
//
// Given this data:
//
// | account.id | closed_on  | balance.id | amount | entered_on |
// |------------|------------|------------|--------|------------|
// | 1          | NULL       | 10         | 100.0  | 2025-01-01 |
// | 1          | NULL       | 11         | 150.0  | 2025-02-01 |
// | 1          | NULL       | 12         | 200.0  | 2025-03-01 |
// | 2          | 2025-01-15 | 20         | 500.0  | 2025-01-01 |
// | 3          | NULL       | 30         | 300.0  | 2025-02-15 |
//
// Result:
//
// | account_id | balance_id | balance | entered_on |
// |------------|------------|---------|------------|
// | 1          | 12         | 200.0   | 2025-03-01 |
// | 3          | 30         | 300.0   | 2025-02-15 |
//
// - Account 1: Only the latest balance (id=12) is returned
// - Account 2: Excluded because closed_on is not NULL
// - Account 3: Has only one balance, so that one is returned
pub fn get_open_accounts_with_latest_balance(
    conn: &mut SqliteConnection,
) -> QueryResult<Vec<LatestBalanceRow>> {
    use diesel::{RunQueryDsl, sql_query};

    // Wrap in a transaction for consistency (read-only, but maintains isolation)
    conn.transaction(|conn| {
        let results = sql_query(
            "SELECT a.id as account_id, a.account_type, a.name as account_name,
                    b.id as balance_id, b.amount, b.entered_on
             FROM account a
             INNER JOIN balance b ON b.account_id = a.id
             WHERE a.closed_on IS NULL
               AND b.entered_on = (
                   SELECT MAX(b2.entered_on)
                   FROM balance b2
                   WHERE b2.account_id = a.id
               )
             ORDER BY b.amount DESC",
        )
        .load::<LatestBalanceRow>(conn)?;

        Ok(results)
    })
}

// Budget item with its recurrence and account allocation info.
//
// This struct represents a denormalized view that joins budget_item with its
// optional recurrence and account allocations in a single query.
#[derive(Debug, QueryableByName, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BudgetItemWithAllocationRow {
    #[diesel(sql_type = Integer)]
    pub item_id: i32,
    #[diesel(sql_type = Text)]
    pub item_name: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub item_description: Option<String>,
    #[diesel(sql_type = Float)]
    pub item_amount: f32,
    #[diesel(sql_type = Nullable<Integer>)]
    pub item_recurrence_id: Option<i32>,
    #[diesel(sql_type = Nullable<Integer>)]
    pub item_recurrence_dt_start: Option<UnixTimestamp>,
    #[diesel(sql_type = Nullable<Text>)]
    pub item_recurrence_rule: Option<RecurrenceRule>,
    #[diesel(sql_type = Nullable<Integer>)]
    pub account_id: Option<i32>,
    #[diesel(sql_type = Nullable<Integer>)]
    pub allocation_percentage: Option<i32>,
}

pub type BudgetData = (
    (cobblepot_data_store::Budget, Option<i32>, Option<UnixTimestamp>, Option<RecurrenceRule>),
    Vec<BudgetItemWithAllocationRow>,
);

// Retrieves budget items with their recurrence and account allocations in a single query.
//
// # SQL Query Breakdown
//
// 1. `FROM budget_item bi`
//    - Base table containing the budget items
//
// 2. `LEFT JOIN budget_recurrence bir ON bir.id = bi.budget_recurrence_id`
//    - Joins optional recurrence rules for each budget item
//    - LEFT JOIN ensures items without recurrence are still included
//
// 3. `LEFT JOIN budget_item_account bia ON bia.budget_item_id = bi.id`
//    - Joins the junction table that links budget items to accounts
//    - LEFT JOIN ensures items without account allocations are included
//    - This is a one-to-many relationship: one item can have multiple allocations
//
// 4. `LEFT JOIN account a ON a.id = bia.account_id`
//    - Joins the account table to get account names
//
// # Result Structure
//
// The query returns denormalized rows. Items with multiple account allocations
// will appear multiple times (once per allocation). Items without allocations
// appear once with NULL allocation fields.
//
// # Example
//
// Given this data:
//
// | budget_item.id | name      | budget_item_account (account_id, %) |
// |----------------|-----------|-------------------------------------|
// | 1              | Rent      | (10, 100)                           |
// | 2              | Groceries | (10, 50), (11, 50)                  |
// | 3              | Gas       | (none)                              |
//
// Result:
//
// | item_id | item_name | account_id | account_name | allocation_percentage |
// |---------|-----------|------------|--------------|----------------------|
// | 1       | Rent      | 10         | Checking     | 100                  |
// | 2       | Groceries | 10         | Checking     | 50                   |
// | 2       | Groceries | 11         | Savings      | 50                   |
// | 3       | Gas       | NULL       | NULL         | NULL                 |
//
// The caller should group results by `item_id` to reconstruct the one-to-many
// relationship between budget items and their account allocations.
pub fn get_budget_data(mut conn: SqliteConnection, budget_id: i32) -> QueryResult<BudgetData> {
    conn.transaction(|conn| {
        use cobblepot_data_store::{
            Budget,
            schema::{budget, budget_recurrence},
        };
        use diesel::{
            ExpressionMethods, NullableExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
            sql_query,
        };
        let budget_with_recurrence = budget::table
            .left_outer_join(budget_recurrence::table)
            .select((
                Budget::as_select(),
                budget_recurrence::id.nullable(),
                budget_recurrence::dt_start.nullable(),
                budget_recurrence::recurrence_rule.nullable(),
            ))
            .filter(budget::id.eq(budget_id))
            .first::<(Budget, Option<i32>, Option<UnixTimestamp>, Option<RecurrenceRule>)>(conn)?;
        let budget_items_with_allocations = sql_query(
            "SELECT
            bi.id as item_id,
            bi.name as item_name,
            bi.description as item_description,
            bi.amount as item_amount,
            bir.id as item_recurrence_id,
            bir.dt_start as item_recurrence_dt_start,
            bir.recurrence_rule as item_recurrence_rule,
            bia.account_id,
            bia.allocation_percentage
        FROM budget_item bi
        LEFT JOIN budget_recurrence bir ON bir.id = bi.budget_recurrence_id
        LEFT JOIN budget_item_account bia ON bia.budget_item_id = bi.id
        WHERE bi.budget_id = ?
        ORDER BY bi.id, bia.account_id",
        )
        .bind::<Integer, _>(budget_id)
        .load::<BudgetItemWithAllocationRow>(conn)?;
        Ok((budget_with_recurrence, budget_items_with_allocations))
    })
}
