use clap::ValueEnum;
use num_enum::{FromPrimitive, IntoPrimitive};
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
