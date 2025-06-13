use std::str::FromStr;

use chrono::{DateTime, Month, NaiveDateTime, TimeZone, Utc};
use rrule::{Frequency, NWeekday, RRule, RRuleError, RRuleSet, Tz, Validated, Weekday};
use serde::{Deserialize, Serialize};

use crate::shared::RecurringStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recurrance {
    pub dt_start: DateTime<Utc>,
    freq: Frequency,
    interval: u16,
    count: u8,
    until: Option<DateTime<Utc>>,
    week_start: Option<Weekday>,
    by_set_pos: Option<Vec<i32>>,
    by_month: Option<Vec<u8>>,
    by_month_day: Option<Vec<i8>>,
    by_year_day: Option<Vec<i16>>,
    by_week_no: Option<Vec<i8>>,
    by_weekday: Option<Vec<NWeekday>>,
    by_hour: Option<Vec<u8>>,
    by_minute: Option<Vec<u8>>,
    by_second: Option<Vec<u8>>,
}

impl Recurrance {
    pub fn rrule(&self) -> Result<RRule<Validated>, RRuleError> {
        // Convert DateTime<Utc> to DateTime<Tz>
        let tz_dt_start = Tz::UTC.from_utc_datetime(&self.dt_start.naive_utc());

        let mut unvalidated =
            RRule::new(self.freq).interval(self.interval).count(self.count.into());

        if let Some(until) = self.until {
            // Convert DateTime<Utc> to DateTime<Tz>
            let tz_until = Tz::UTC.from_utc_datetime(&until.naive_utc());
            unvalidated = unvalidated.until(tz_until);
        }

        if let Some(week_start) = self.week_start {
            unvalidated = unvalidated.week_start(week_start);
        }

        if let Some(by_set_pos) = &self.by_set_pos {
            unvalidated = unvalidated.by_set_pos(by_set_pos.clone());
        }

        if let Some(by_month) = &self.by_month {
            // Convert u8 values to Month values
            let months: Vec<Month> =
                by_month.iter().filter_map(|&m| Month::try_from(m).ok()).collect();

            if !months.is_empty() {
                unvalidated = unvalidated.by_month(&months);
            }
        }

        if let Some(by_month_day) = &self.by_month_day {
            unvalidated = unvalidated.by_month_day(by_month_day.clone());
        }

        if let Some(by_year_day) = &self.by_year_day {
            unvalidated = unvalidated.by_year_day(by_year_day.clone());
        }

        if let Some(by_week_no) = &self.by_week_no {
            unvalidated = unvalidated.by_week_no(by_week_no.clone());
        }

        if let Some(by_weekday) = &self.by_weekday {
            unvalidated = unvalidated.by_weekday(by_weekday.clone());
        }

        if let Some(by_hour) = &self.by_hour {
            unvalidated = unvalidated.by_hour(by_hour.clone());
        }

        if let Some(by_minute) = &self.by_minute {
            unvalidated = unvalidated.by_minute(by_minute.clone());
        }

        if let Some(by_second) = &self.by_second {
            unvalidated = unvalidated.by_second(by_second.clone());
        }

        unvalidated.validate(tz_dt_start)
    }
}

pub fn recurrance_status(
    rrule: String,
    dt_start: NaiveDateTime,
    closed: bool,
) -> Result<RecurringStatus, RRuleError> {
    if closed {
        return Ok(RecurringStatus::Closed);
    }

    let unvalidated = RRule::from_str(rrule.as_str())?;
    let tz_dt_start = Tz::UTC.from_utc_datetime(&dt_start);
    let validated = unvalidated.validate(tz_dt_start)?;
    let rrule_iter = RRuleSet::new(tz_dt_start).rrule(validated);
    let recurrance_result = rrule_iter.all(u8::MAX.into());
    if recurrance_result.limited {
        println!("Possibly false positive since entire set not included")
    }

    let today = Utc::now();
    for date in recurrance_result.dates {
        if date > today {
            return Ok(RecurringStatus::Ongoing);
        }
    }
    Ok(RecurringStatus::Completed)
}
