use clap::ValueEnum;
use cobblepot_data_store::{BudgetRecurrence, RecurrenceRule, UnixTimestamp, schema};
use diesel::{Insertable, QueryResult, RunQueryDsl, SqliteConnection, dsl::insert_into};
use num_enum::{FromPrimitive, IntoPrimitive};
use rrule::{RRuleResult, RRuleSet, Tz};
use serde::{Deserialize, Serialize};

/// ValueEnum for AccountType
#[derive(
    Debug,
    ValueEnum,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Default,
    FromPrimitive,
    IntoPrimitive,
)]
#[repr(i32)]
pub enum AccountType {
    #[default]
    Asset = 0,
    Liability = 1,
}

/// Custom parser for NaiveDateTime that accepts dates in YYYY-MM-DD format
/// and sets the time to 00:00:00
pub fn parse_date(s: &str) -> Result<chrono::NaiveDateTime, String> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map(|date| date.and_hms_opt(0, 0, 0).unwrap())
        .map_err(|e| format!("Invalid date format. Expected YYYY-MM-DD, got '{}': {}", s, e))
}

pub fn format_money_usd(value: f32) -> String {
    let is_negative = value < 0.0;
    let abs_value = value.abs();
    let whole = abs_value.trunc() as u64;
    let cents = ((abs_value.fract() * 100.0).round() as u64) % 100;

    // Add commas for thousands separators
    let whole_str = whole.to_string();
    let with_commas: String = whole_str
        .chars()
        .rev()
        .enumerate()
        .flat_map(|(i, c)| {
            if i > 0 && i % 3 == 0 {
                vec![',', c]
            } else {
                vec![c]
            }
        })
        .collect::<String>()
        .chars()
        .rev()
        .collect();

    let sign = if is_negative {
        "-"
    } else {
        ""
    };
    format!("{}${}.{:02}", sign, with_commas, cents)
}

/// Validates an rrule against a dt_start and returns the validated RecurrenceRule
pub fn validate_rrule(dt_start: &UnixTimestamp, rrule: &mut RecurrenceRule) -> RecurrenceRule {
    let rrule_dt_start = dt_start.inner().and_local_timezone(Tz::UTC).unwrap();
    rrule
        .validate(rrule_dt_start)
        .inspect_err(|e| log::error!("Could not validate rrule: {:?}", e))
        .expect("Invalid rrule")
        .into()
}

#[derive(Debug, Insertable)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name = schema::budget_recurrence)]
pub struct NewBudgetRecurrence {
    pub dt_start: UnixTimestamp,
    pub recurrence_rule: RecurrenceRule,
    pub budget_id: Option<i32>,
    pub budget_item_id: Option<i32>,
}

/// Creates a new BudgetRecurrence and returns its id
pub fn create_budget_recurrence(
    conn: &mut SqliteConnection,
    dt_start: UnixTimestamp,
    mut rrule: RecurrenceRule,
    budget_id: Option<i32>,
    budget_item_id: Option<i32>,
) -> QueryResult<i32> {
    let validated = validate_rrule(&dt_start, &mut rrule);

    let recurrence = insert_into(schema::budget_recurrence::table)
        .values(NewBudgetRecurrence {
            dt_start,
            recurrence_rule: validated,
            budget_id,
            budget_item_id,
        })
        .get_result::<BudgetRecurrence>(conn)?;

    Ok(recurrence.id)
}

pub fn zip_all<A, B, C>(a: Option<A>, b: Option<B>, c: Option<C>) -> Option<(A, B, C)> {
    match a.zip(b) {
        Some(e) => match c {
            Some(r) => Some((e.0, e.1, r)),
            None => None,
        },
        None => None,
    }
}

pub fn get_next_occurences(
    rule: rrule::RRule<rrule::Validated>,
    dt_start: &UnixTimestamp,
) -> RRuleResult {
    log::trace!("Starting next occurences {}", rule);
    let temp_set =
        RRuleSet::new(dt_start.inner().and_utc().with_timezone(&rrule::Tz::UTC)).rrule(rule);
    // let temp_set = RRuleSet::from_str(rule.to_string().as_str()).unwrap();
    let after_now = temp_set.after(chrono::Utc::now().with_timezone(&rrule::Tz::UTC));
    after_now.all(5)
}
