use std::collections::HashMap;

use chrono::{DateTime, Utc};
use rusty_money::iso::Currency;
use rusty_money::Money;

use self::account_type::AccountType;
use self::balance::Balance;
use self::code::AccountCode;
use self::transaction::Transaction;

pub mod account_type;
pub mod balance;
mod code;
pub mod transaction;

struct Account {
    name: String,
    description: String,
    code: AccountCode,
    opened: DateTime<Utc>,
    closed: Option<DateTime<Utc>>,
    balances: Vec<Balance>,
    transactions: HashMap<DateTime<Utc>, Transaction>,
}

impl Account {
    pub fn create_new_account(name: String, description: String, code: AccountCode) -> Account {
        Account {
            name,
            description,
            code,
            opened: chrono::offset::Utc::now(),
            closed: None,
            balances: Vec::new(),
            transactions: HashMap::new(),
        }
    }

    pub fn load_account(
        name: String,
        description: String,
        code: AccountCode,
        opened: DateTime<Utc>,
        closed: Option<DateTime<Utc>>,
        balances: Vec<Balance>,
        transactions: HashMap<DateTime<Utc>, Transaction>,
    ) -> Account {
        Account { name, description, code, opened, closed, balances, transactions }
    }

    pub fn add_transaction(&mut self, transaction: Transaction, date: Option<DateTime<Utc>>) {
        let date_key = date.or_else(|| Some(Utc::now())).unwrap();
        if self.transactions.contains_key(&date_key) {
            panic!("Transaction already exists for date {}", date_key);
        }
        self.transactions.insert(date_key, transaction);
    }

    pub fn get_transaction(&self, date: DateTime<Utc>) -> Option<&Transaction> {
        self.transactions.get(&date)
    }

    pub fn get_transaction_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<&Transaction> {
        let mut transactions = Vec::new();
        for (date, transaction) in &self.transactions {
            if date >= &start && date <= &end {
                transactions.push(transaction);
            }
        }
        transactions
    }

    /// Calculate the balance of the account for a given currency
    /// Checks the last balance and calculates the new balance based on the transactions since
    /// then
    pub fn calculate_balance(&mut self, currency: &'static Currency) {
        let account_type = self.code.extract_account_type();
        let mut balance = Balance::new(Money::from_major(0, currency), None);
        let previous_balance = self.balances.last();

        match previous_balance {
            Some(previous_balance) => {
                let transactions_since = self.get_transaction_range(
                    previous_balance.calculated_on(),
                    chrono::offset::Utc::now(),
                );
                transactions_since.iter().for_each(|transaction| {
                    balance.calculate_post_transaction(&account_type, transaction);
                });
            },
            None => {
                self.transactions.iter().for_each(|(_, transaction)| {
                    balance.calculate_post_transaction(&account_type, transaction);
                });
            },
        }

        self.balances.push(balance);
    }

    pub fn push_balance(&mut self, balance: Balance) {
        self.balances.push(balance);
    }
}

// TODO: Add tests
