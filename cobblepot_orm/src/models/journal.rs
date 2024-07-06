use diesel::prelude::*;
use rusty_money::iso::Currency;
use rusty_money::Money;

#[derive(Queryable, Selectable, Identifiable, Insertable, QueryableByName)]
#[diesel(table_name = crate::schema::journal)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(entry_code))]
pub struct Journal {
    entry_code: i32,
    pub memo: String,
    pub account_code: i32,
    pub transaction_code: i32,
}

#[derive(Queryable, Selectable, Insertable, QueryableByName)]
#[diesel(table_name = crate::schema::journal)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewJournalEntry {
    pub memo: String,
    pub account_code: i32,
    pub transaction_code: i32,
}

#[derive(Associations, Identifiable, QueryableByName, Queryable, Selectable)]
#[diesel(table_name = crate::schema::transaction_event)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Journal, foreign_key=transaction_code))]
#[diesel(primary_key(transaction_code))]
pub struct JournalTransaction {
    transaction_code: i32,
    amount: i32,
    currency: String,
    created_on: chrono::NaiveDateTime,
}

impl JournalTransaction {
    pub fn transaction_code(&self) -> i32 {
        self.transaction_code.clone()
    }
    pub fn value(&self) -> Money<'static, Currency> {
        let currency_code = rusty_money::iso::find_by_num_code(&self.currency).unwrap();
        rusty_money::Money::from_minor(self.amount.into(), currency_code)
    }
    pub fn created_on(&self) -> chrono::NaiveDateTime {
        self.created_on.clone()
    }
}

#[derive(Insertable, AsChangeset, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::transaction_event)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewJournalTransaction {
    amount: i32,
    currency: String,
}
impl NewJournalTransaction {
    pub fn new(value: Money<'static, Currency>) -> NewJournalTransaction {
        NewJournalTransaction {
            amount: value.amount().into(),
            currency: value.currency().to_string(),
        }
    }
}
