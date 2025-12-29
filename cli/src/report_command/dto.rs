use diesel::{
    Connection, QueryResult, QueryableByName, SqliteConnection,
    sql_types::{Float, Integer, Text, Timestamp},
};

// Intermediate struct for deserializing raw SQL results.
// Field names must match the column aliases in the SELECT clause.
#[derive(QueryableByName)]
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
    #[diesel(sql_type = Timestamp)]
    pub entered_on: chrono::NaiveDateTime,
}

// Retrieves the latest balance entry for each open account using raw SQL.
//
// This function uses Diesel's `sql_query` to execute a raw SQL statement because
// Diesel's type-safe query builder has limitations with correlated subqueries
// in join contexts.
//
// # SQL Query Breakdown
//
// ```sql
// SELECT a.id as account_id, b.id as balance_id, b.amount, b.entered_on
// FROM account a
// INNER JOIN balance b ON b.account_id = a.id
// WHERE a.closed_on IS NULL
//   AND b.entered_on = (
//       SELECT MAX(b2.entered_on)
//       FROM balance b2
//       WHERE b2.account_id = a.id
//   )
// ```
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
//
// # QueryableByName
//
// The `LatestBalanceRow` struct uses `#[derive(QueryableByName)]` which maps
// SQL result columns by name (not position) to struct fields. Each field requires
// a `#[diesel(sql_type = ...)]` annotation to tell Diesel how to deserialize
// the column value.
pub fn get_open_accounts_with_latest_balance(
    mut conn: SqliteConnection,
) -> QueryResult<Vec<LatestBalanceRow>> {
    use diesel::{RunQueryDsl, sql_query};

    // Wrap in a transaction for consistency (read-only, but maintains isolation)
    conn.transaction(|conn| {
        let results = sql_query(
            "SELECT a.id as account_id, a.account_type, a.name as account_name, b.id as balance_id, b.amount, b.entered_on
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

// Two-step approach using Diesel's type-safe query builder:
// 1. Get all open account IDs
// 2. For each account, query the latest balance entry
// Slower option N+1 but type safe
// fn get_open_accounts_with_latest_balance_two_step(
//     mut conn: SqliteConnection,
// ) -> QueryResult<Vec<AccountBalance>> {
//     use cobblepot_data_store::schema::{account, balance};
//     use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
//
//     conn.transaction(|conn| {
//         // Step 1: Get all open account IDs
//         let open_account_ids: Vec<i32> =
//             account::table.filter(account::closed_on.is_null()).select(account::id).load(conn)?;
//
//         // Step 2: For each account, get the latest balance entry
//         let mut results = Vec::new();
//         for account_id in open_account_ids {
//             let latest_balance: Option<(i32, f32, chrono::NaiveDateTime)> = balance::table
//                 .filter(balance::account_id.eq(account_id))
//                 .order(balance::entered_on.desc())
//                 .select((balance::id, balance::amount, balance::entered_on))
//                 .first(conn)
//                 .optional()?;
//
//             if let Some((balance_id, amount, entered_on)) = latest_balance {
//                 results.push(AccountBalance {
//                     account_id,
//                     balance_id,
//                     balance: amount,
//                     entered_on,
//                 });
//             }
//         }
//
//         Ok(results)
//     })
// }
