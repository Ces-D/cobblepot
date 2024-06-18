use chrono::{DateTime, Utc};
use diesel::prelude::*;
use rusty_money::iso::Currency;
use rusty_money::{Money, MoneyError};

#[derive(Queryable, Selectable, Identifiable, Insertable)]
#[diesel(table_name = crate::schema::journal)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(entry_code))]
pub struct Journal {
    entry_code: i32,
    pub memo: String,
    account_code: i32,
    transaction_code: i32,
}

impl Journal {}

#[derive(Selectable, Identifiable, Associations, Insertable)]
#[diesel(table_name = crate::schema::transaction_event)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Journal, foreign_key=transaction_code))]
#[diesel(primary_key(transaction_code))]
pub struct TransactionEvent {
    transaction_code: i32,
    amount: String,
    currency: String,
    created_on: String,
}

impl TransactionEvent {
    pub fn transaction_code(&self) -> i32 {
        self.transaction_code.clone()
    }
    pub fn value(&self) -> Result<Money<'static, Currency>, MoneyError> {
        let currency_code = rusty_money::iso::find_by_num_code(&self.currency).unwrap();
        rusty_money::Money::from_str(self.amount.as_str(), currency_code)
    }
    pub fn created_on(&self) -> DateTime<Utc> {
        self.created_on.parse().unwrap()
    }
}
