use super::recurrance::Recurrance;
use crate::{schema::recurring_transactions, shared::AccountType};
use chrono::NaiveDateTime;
use cli_docs_macro::CliDocs;
use diesel::prelude::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, CliDocs)]
pub struct CliOpenRecurringTransaction {
    #[cli_docs(description = "Name of the recurring transaction")]
    pub name: String,
    #[cli_docs(description = "Description of the recurring transaction")]
    pub description: Option<String>,
    #[cli_docs(description = "Amount of the recurring transaction")]
    pub amount: f32,
    #[cli_docs(
        default = "0",
        description = "Impact of the recurring transaction on the account. Asset=0, Liability=1"
    )]
    pub account_type: Option<AccountType>,
    pub recurrance: Recurrance,
    #[cli_docs(description = "ID of the account associated with the recurring transaction")]
    pub account_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, CliDocs)]
pub struct CliCloseRecurringTransaction {
    #[cli_docs(description = "ID of the recurring transaction to close")]
    pub id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=recurring_transactions)]
pub struct InsertableRecurringTransaction {
    pub name: String,
    pub description: Option<String>,
    pub account_type: i32,
    pub amount: f32,
    pub rrule: String,
    pub start_date: NaiveDateTime,
    pub closed: bool,
    pub account_id: i32,
}

impl TryFrom<CliOpenRecurringTransaction> for InsertableRecurringTransaction {
    type Error = rrule::RRuleError;

    fn try_from(value: CliOpenRecurringTransaction) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            description: value.description,
            account_type: value.account_type.unwrap_or(AccountType::Asset) as i32,
            amount: value.amount,
            rrule: value.recurrance.rrule()?.to_string(),
            start_date: value.recurrance.dt_start.naive_utc(),
            closed: false,
            account_id: value.account_id,
        })
    }
}

#[derive(Debug, Queryable, Identifiable, Serialize)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name=recurring_transactions)]
pub struct RecurringTransaction {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub account_type: i32,
    pub amount: f32,
    pub rrule: String,
    pub start_date: NaiveDateTime,
    pub closed: bool,
    pub account_id: i32,
}
