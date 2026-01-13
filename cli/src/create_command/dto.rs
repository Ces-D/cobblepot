use crate::shared::AccountType;
use chrono::Local;
use cobblepot_data_store::{RecurrenceRule, UnixTimestamp};
use diesel::{Connection, Insertable, QueryResult, SqliteConnection};
use rrule::Tz;

/// Represents a new account to be inserted into the database.
#[derive(Debug, Insertable)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=cobblepot_data_store::schema::account)]
struct NewAccount {
    pub name: String,
    pub description: Option<String>,
    pub owner: String,
    pub account_type: i32,
    pub opened_on: UnixTimestamp,
    pub closed_on: Option<UnixTimestamp>,
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
                opened_on: UnixTimestamp(opened_on.unwrap_or_else(|| Local::now().naive_utc())),
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
    pub entered_on: UnixTimestamp,
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
    let entered_on = UnixTimestamp(entered_on.unwrap_or_else(|| Local::now().naive_utc()));
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
                        entered_on.inner().format("%v").to_string()
                    )
                }),
            })
            .get_result::<Balance>(conn)?;
        Ok((res.id, res.memo))
    })
}

#[derive(Debug, Insertable)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=cobblepot_data_store::schema::budget)]
struct NewBudget {
    name: String,
    description: Option<String>,
    anticipated_amount: f32,
    budget_recurrence_id: Option<i32>,
}

#[derive(Debug, Insertable)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=cobblepot_data_store::schema::budget_recurrence)]
struct NewBudgetRecurrence {
    dt_start: UnixTimestamp,
    recurrence_rule: RecurrenceRule,
    budget_id: Option<i32>,
    budget_item_id: Option<i32>,
}

pub fn create_new_budget(
    mut conn: SqliteConnection,
    name: String,
    description: Option<String>,
    anticipated_amount: f32,
    r_rule: Option<(chrono::NaiveDateTime, String)>,
) -> QueryResult<(i32, String)> {
    use cobblepot_data_store::{
        Budget, BudgetRecurrence,
        schema::{budget, budget_recurrence},
    };
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, dsl::insert_into};
    match r_rule {
        Some((dt_start, unvalidated_str)) => conn.transaction(|conn| {
            let mut unvalidated_rrule = RecurrenceRule::new(unvalidated_str);
            let rrule_dt_start = dt_start.clone().and_local_timezone(Tz::UTC).unwrap();
            let validated: RecurrenceRule = unvalidated_rrule
                .validate(rrule_dt_start)
                .inspect_err(|e| log::error!("Could not validate rrule: {:?}", e))
                .expect("Invalid rrule")
                .into();
            let res = insert_into(budget::table)
                .values(NewBudget {
                    name,
                    description,
                    anticipated_amount,
                    budget_recurrence_id: None,
                })
                .get_result::<Budget>(conn)?;
            let b_recurrence = insert_into(budget_recurrence::table)
                .values(NewBudgetRecurrence {
                    dt_start: UnixTimestamp::new(dt_start),
                    recurrence_rule: validated,
                    budget_id: Some(res.id),
                    budget_item_id: None,
                })
                .get_result::<BudgetRecurrence>(conn)?;
            diesel::update(budget::table.filter(budget::id.eq(res.id)))
                .set(budget::budget_recurrence_id.eq(b_recurrence.id))
                .execute(conn)?;
            Ok((res.id, res.name))
        }),
        None => conn.transaction(|conn| {
            let res = insert_into(budget::table)
                .values(NewBudget {
                    name,
                    description,
                    anticipated_amount,
                    budget_recurrence_id: None,
                })
                .get_result::<Budget>(conn)?;
            Ok((res.id, res.name))
        }),
    }
}

#[derive(Debug, Insertable)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=cobblepot_data_store::schema::budget_item)]
struct NewBudgetItem {
    budget_id: i32,
    name: String,
    description: Option<String>,
    amount: f32,
    budget_recurrence_id: Option<i32>,
}

pub fn create_new_budget_item(
    mut conn: SqliteConnection,
    name: String,
    description: Option<String>,
    amount: f32,
    r_rule: Option<(chrono::NaiveDateTime, String)>,
    budget_id: i32,
) -> QueryResult<(i32, String)> {
    use cobblepot_data_store::{
        BudgetItem, BudgetRecurrence,
        schema::{budget_item, budget_recurrence},
    };
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, dsl::insert_into};
    match r_rule {
        Some((dt_start, unvalidated_str)) => conn.transaction(|conn| {
            let mut unvalidated_rrule = RecurrenceRule::new(unvalidated_str);
            let rrule_dt_start = dt_start.clone().and_local_timezone(Tz::UTC).unwrap();
            let validated: RecurrenceRule = unvalidated_rrule
                .validate(rrule_dt_start)
                .inspect_err(|e| log::error!("Could not validate rrule: {:?}", e))
                .expect("Invalid rrule")
                .into();
            let res = insert_into(budget_item::table)
                .values(NewBudgetItem {
                    budget_id,
                    name,
                    description,
                    amount,
                    budget_recurrence_id: None,
                })
                .get_result::<BudgetItem>(conn)?;
            let b_recurrence = insert_into(budget_recurrence::table)
                .values(NewBudgetRecurrence {
                    dt_start: UnixTimestamp::new(dt_start),
                    recurrence_rule: validated,
                    budget_id: None,
                    budget_item_id: Some(res.id),
                })
                .get_result::<BudgetRecurrence>(conn)?;

            diesel::update(budget_item::table.filter(budget_item::id.eq(res.id)))
                .set(budget_item::budget_recurrence_id.eq(b_recurrence.id))
                .execute(conn)?;
            Ok((res.id, res.name))
        }),
        None => conn.transaction(|conn| {
            let res = insert_into(budget_item::table)
                .values(NewBudgetItem {
                    budget_id,
                    name,
                    description,
                    amount,
                    budget_recurrence_id: None,
                })
                .get_result::<BudgetItem>(conn)?;
            Ok((res.id, res.name))
        }),
    }
}

#[derive(Debug, Insertable)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=cobblepot_data_store::schema::budget_item_account)]
pub struct NewBudgetItemAccount {
    budget_item_id: i32,
    account_id: i32,
    allocation_percentage: Option<i32>,
}

pub fn create_new_budget_item_account_meta(
    mut conn: SqliteConnection,
    budget_item_id: i32,
    account_id: i32,
    percentage: Option<i32>,
) -> QueryResult<usize> {
    use cobblepot_data_store::schema::budget_item_account;
    use diesel::{RunQueryDsl, dsl::insert_into};
    conn.transaction(|conn| {
        let res = insert_into(budget_item_account::table)
            .values(NewBudgetItemAccount {
                budget_item_id,
                account_id,
                allocation_percentage: percentage,
            })
            .execute(conn)?;
        Ok(res)
    })
}
