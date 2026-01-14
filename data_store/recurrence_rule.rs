use std::str::FromStr;

use chrono::DateTime;
use diesel::{
    AsExpression, FromSqlRow,
    backend::Backend,
    deserialize::{self, FromSql},
    serialize::{self, Output, ToSql},
    sql_types::Text,
    sqlite::Sqlite,
};
use rrule::{RRule, RRuleError, RRuleSet, Tz, Unvalidated, Validated};
use serde::{Deserialize, Serialize};

/// A wrapper type that converts between Rrule and String (Text in SQLite)
/// Be sure to validate prior to storing in db
#[derive(Debug, Clone, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = Text)]
pub struct RecurrenceRule(String);

impl RecurrenceRule {
    /// Stores an unvalidated string used only for converting FromSql
    pub fn new(r_rule: String) -> Self {
        Self(r_rule)
    }
    pub fn try_inner(&self) -> Result<RRule<Unvalidated>, RRuleError> {
        RRule::from_str(&self.0)
    }
    /// Validates the recurrence rule with the following constraints:
    /// - COUNT is capped at 100 (defaults to 30 if not set)
    /// - UNTIL is always present (calculated from occurrences if not provided)
    pub fn validate(&mut self, dt_start: DateTime<Tz>) -> Result<RRule<Validated>, RRuleError> {
        const MAX_COUNT: u32 = 100;
        let mut unvalidated = RRule::from_str(&self.0)?;
        const DEFAULT_COUNT: u32 = 30;
        // Ensure count never exceeds MAX_COUNT, default to MAX_COUNT if not set
        let effective_count =
            unvalidated.get_count().map(|c| c.min(MAX_COUNT)).unwrap_or_else(|| {
                log::info!("Setting COUNT for recurrence to DEFAULT: {}", DEFAULT_COUNT);
                DEFAULT_COUNT
            });
        unvalidated = unvalidated.count(effective_count);
        // Ensure UNTIL is always set by calculating it from occurrences if missing
        if unvalidated.get_until().is_none() {
            let temp_set: RRuleSet = unvalidated.clone().build(dt_start)?;
            if let Some(last) = temp_set.into_iter().last() {
                log::info!("Setting UNTIL for recurrence to {}", last.format("%d/%m/%Y %H:%M"));
                unvalidated = unvalidated.until(last);
            }
        }

        let validated = unvalidated.validate(dt_start)?;
        self.0 = validated.to_string();
        Ok(validated)
    }
}

impl From<RRule<Validated>> for RecurrenceRule {
    fn from(value: RRule<Validated>) -> Self {
        Self(value.to_string())
    }
}

impl<DB> FromSql<Text, DB> for RecurrenceRule
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let rrule_str = String::from_sql(bytes)?;
        let r_rule = Self::new(rrule_str);
        Ok(r_rule)
    }
}

impl ToSql<Text, Sqlite> for RecurrenceRule {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(self.0.clone());
        Ok(serialize::IsNull::No)
    }
}
