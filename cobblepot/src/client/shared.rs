pub mod sql {
    use diesel::{
        deserialize::FromSql,
        serialize::{Output, ToSql},
        sql_types::Integer,
        sqlite::{Sqlite, SqliteValue},
    };

    #[derive(
        Debug,
        Clone,
        Copy,
        diesel::AsExpression,
        diesel::FromSqlRow,
        serde::Serialize,
        serde::Deserialize,
        clap::ValueEnum,
        strum::Display,
        PartialEq,
    )]
    #[diesel(sql_type = Integer)]
    #[repr(i32)]
    pub enum AccountType {
        Asset = 0,
        Liability = 1,
    }

    impl FromSql<Integer, Sqlite> for AccountType {
        fn from_sql(bytes: SqliteValue) -> diesel::deserialize::Result<Self> {
            let t = i32::from_sql(bytes)?;
            match t {
                0 => Ok(AccountType::Asset),
                1 => Ok(AccountType::Liability),
                _ => Err("Invalid account type".into()),
            }
        }
    }

    impl ToSql<Integer, Sqlite> for AccountType {
        fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
            match self {
                AccountType::Asset => <i32 as ToSql<Integer, Sqlite>>::to_sql(&0, out),
                AccountType::Liability => <i32 as ToSql<Integer, Sqlite>>::to_sql(&1, out),
            }
        }
    }
}

pub mod cli {
    use chrono::{NaiveDate, NaiveDateTime, Utc};

    pub fn parse_iso8601_variant_datetime(
        s: &str,
    ) -> Result<NaiveDateTime, chrono::format::ParseError> {
        match NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f") {
            Ok(dt) => Ok(dt),
            Err(_) => {
                let d = NaiveDate::parse_from_str(s, "%Y-%m-%d")?;
                let dt = d.and_hms_opt(0, 0, 0).expect("Midnight is a valid time");
                Ok(dt)
            },
        }
    }

    /// Parse a string into the format "YYYY-MM-DD HH:MM:SS.SSS"
    pub fn parse_iso8601_variant_date(s: &str) -> Result<String, chrono::format::ParseError> {
        match NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f") {
            Ok(dt) => Ok(dt.to_string()),
            Err(_) => {
                let d = NaiveDate::parse_from_str(s, "%Y-%m-%d")?;
                let dt = d.and_hms_opt(0, 0, 0).expect("Midnight is a valid time");
                Ok(dt.to_string())
            },
        }
    }

    /// Get the current date as a Unix timestamp
    pub fn default_iso8601_variant_date() -> String {
        // Return it as a DateTime<Utc> at midnight
        let now = Utc::now();
        now.format("%Y-%m-%d %H:%M:%S%.f").to_string()
    }

    /// A standard help message for ISO8601 dates
    pub const ISO8601_DATE_LONG_HELP: &str =
        "Accepted formats include: 'YYYY-MM-DD' and 'YYYY-MM-DD HH:MM:SS.SSS'";
}

pub mod report {
    use tabled::derive::display;

    #[derive(Debug, Clone, serde::Serialize, tabled::Tabled)]
    #[tabled(display(Option, "display::option", "Unknown"))]
    pub struct ReportItem {
        pub name: String,
        pub description: Option<String>,
        pub owner: String,
        pub balance: f32,
    }

    #[derive(Debug, Clone, serde::Serialize)]
    pub struct BalanceSheet {
        pub from: String,
        pub to: String,
        pub current_assets: Vec<ReportItem>,
        pub current_liabilities: Vec<ReportItem>,
        pub non_current_assets: Vec<ReportItem>,
        pub non_current_liabilities: Vec<ReportItem>,
    }

    #[derive(Debug, Clone, serde::Serialize)]
    pub struct AccountLevelAnalytics {
        pub age_in_days: i32,
    }

    /// The balance milestones to track
    pub const MILESTONE_VALUES: [f32; 14] = [
        100_000_000.0,
        10_000_000.0,
        5_000_000.0,
        1_000_000.0,
        500_000.0,
        250_000.0,
        200_000.0,
        150_000.0,
        100_000.0,
        50_000.0,
        10_000.0,
        5_000.0,
        1_000.0,
        500.0,
    ];

    #[derive(Debug, Clone, serde::Serialize)]
    pub struct FinancialAnalytics {
        pub current_balance: f32,
        pub average_balance: f32,
        pub total_balance_updates: i32,
        pub historical_max_balance: f32,
        pub historical_min_balance: f32,
        /// The balance changes for the last 12 months
        pub past_year_deltas: [Option<f32>; 12],
        /// 500, 1_000, 5_000, 10_000, 50_000, 100_000, 150_000, 200_000, 250_000, 500_000, 1_000_000, 5_000_000, 10_000_000, 100_000_000  balance milestones
        pub balance_milestone_dates: [Option<chrono::NaiveDateTime>; 14],
    }

    #[derive(Debug, Clone, serde::Serialize)]
    pub struct BehavioralAnalytics {
        pub days_since_last_transaction: i32,
    }

    #[derive(Debug, Clone, serde::Serialize)]
    pub struct DeepDiveAnalysis {
        pub account: crate::client::account::AccountDetailed,
        pub account_level: AccountLevelAnalytics,
        pub financial: FinancialAnalytics,
        pub behavioral: BehavioralAnalytics,
    }
}
