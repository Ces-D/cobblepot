use crate::code::AccountCode;
use crate::historical_record::{Balance, Transfer};
use chrono::{DateTime, Utc};

pub enum Frequency {
    Daily(u8),
    Weekly(u8),
    Monthly(u8),
}

pub enum Metadata {
    Asset(AssetMetadata),
    Liability(LiabilityMetadata),
    Income(IncomeMetadata),
    Expense(ExpenseMetadata),
    Equity(EquityMetadata),
}

pub struct AssetMetadata {
    pub name: String,
    pub description: String,
    pub acquisition_date: Option<DateTime<Utc>>,
}

pub struct LiabilityMetadata {
    pub name: String,
    pub description: String,
}

pub struct EquityMetadata {
    pub name: String,
    pub description: String,
    pub issue_date: Option<DateTime<Utc>>,
    pub dividends_paid_percentage: Option<rust_decimal::Decimal>,
}

pub struct ExpenseMetadata {
    pub name: String,
    pub description: String,
    pub frequency: Frequency,
    pub previous_transfer: DateTime<Utc>,
}

pub struct IncomeMetadata {
    pub name: String,
    pub description: String,
    pub frequency: Frequency,
    pub previous_transfer: DateTime<Utc>,
}

pub struct Account {
    pub code: AccountCode,
    pub created: DateTime<Utc>,
    pub closed: bool,
    pub previous_transfer: Option<Transfer>,
    pub balance: Balance,
    pub metadata: Metadata,
}

impl Account {
    pub fn new(
        code: AccountCode,
        created: Option<DateTime<Utc>>,
        balance: Balance,
        metadata: Metadata,
    ) -> Self {
        Account {
            code,
            created: created.unwrap_or(chrono::offset::Utc::now()),
            previous_transfer: None,
            closed: false,
            balance,
            metadata,
        }
    }
}
