use super::recurrance::Reccurance;
use crate::schema::recurring_transactions;
use crate::shared::AccountType;
use diesel::prelude::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliOpenRecurringTransaction {
    pub name: String,
    pub description: Option<String>,
    pub amount: f32,
    pub account_type: Option<AccountType>,
    pub recurrance: Reccurance,
    pub account_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliCloseRecurringTransaction {
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
    pub status: i32,
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
            status: value.recurrance.status()? as i32,
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
    pub status: i32,
    pub account_id: i32,
}
