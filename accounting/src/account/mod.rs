use std::collections::HashMap;

use chrono::{DateTime, Utc};
use rusty_money::iso::Currency;
use rusty_money::Money;

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

    pub fn update_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn close(&mut self) {
        if self.closed.is_some() {
            panic!("Account is already closed");
        }
        self.closed = Some(chrono::offset::Utc::now());
    }

    pub fn add_transaction(&mut self, transaction: Transaction, date: DateTime<Utc>) {
        if self.transactions.contains_key(&date) {
            panic!("Transaction already exists for date {}", date);
        }
        self.transactions.insert(date, transaction);
    }

    pub fn get_transaction(&self, date: DateTime<Utc>) -> Option<&Transaction> {
        self.transactions.get(&date)
    }

    pub fn get_transactions_within(
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

    pub fn push_balance(&mut self, balance: Balance) {
        self.balances.push(balance);
    }

    /// Calculate the balance of the account for a given currency
    /// Checks the last balance and calculates the new balance based on the transactions since
    /// then
    pub fn calculate_balance(&mut self, currency: &'static Currency) {
        let account_type = self.code.extract_account_type();
        let previous_balance = self.balances.last();

        self.push_balance(match previous_balance {
            Some(recent_balance) => {
                let transactions_since = self.get_transactions_within(
                    recent_balance.calculated_on(),
                    chrono::offset::Utc::now(),
                );
                let mut balance = Balance::new(recent_balance.value(), Utc::now());
                for ele in transactions_since {
                    balance.calculate_post_transaction(&account_type, ele);
                }
                balance
            },

            None => {
                let mut balance = Balance::new(Money::from_major(0, currency), Utc::now());
                self.transactions.iter().for_each(|(_, transaction)| {
                    balance.calculate_post_transaction(&account_type, transaction);
                });
                balance
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use rusty_money::iso;

    use super::{transaction::entry::Entry, *};

    fn create_accounts() -> [Account; 2] {
        let asset_account = Account::create_new_account(
            String::from("Asset Account"),
            String::from("Asset Account Description"),
            AccountCode::new(account_type::AccountType::Asset, 0),
        );
        let revenue_account = Account::create_new_account(
            String::from("Expense Account"),
            String::from("Expense description"),
            AccountCode::new(account_type::AccountType::Revenue, 0),
        );

        [asset_account, revenue_account]
    }

    fn create_transaction_dates() -> [DateTime<Utc>; 3] {
        [
            Utc::now().checked_sub_months(chrono::Months::new(2)).unwrap(),
            Utc::now().checked_sub_days(chrono::Days::new(2)).unwrap(),
            Utc::now().checked_sub_days(chrono::Days::new(1)).unwrap(),
        ]
    }

    #[test]
    fn test_account_transactions() {
        let [mut assets, _] = create_accounts();
        let transaction_dates = create_transaction_dates();

        for date in transaction_dates {
            assets.add_transaction(
                Transaction::Credit(Entry::new(
                    Money::from_major(10, iso::USD),
                    String::from("test "),
                )),
                date,
            );
        }

        assert_eq!(assets.transactions.len(), 3);
        assert_ne!(assets.get_transaction(transaction_dates[0]), Option::None);
        assert_eq!(
            assets.get_transactions_within(transaction_dates[0], transaction_dates[1]).len(),
            2
        );
    }

    #[test]
    fn test_account_balances() {
        let [mut assets, mut revenue] = create_accounts();
        let transaction_dates = create_transaction_dates();

        for date in transaction_dates {
            assets.add_transaction(
                Transaction::Credit(Entry::new(
                    Money::from_major(10, iso::USD),
                    String::from("test today"),
                )),
                date,
            );
            revenue.add_transaction(
                Transaction::Credit(Entry::new(
                    Money::from_major(10, iso::USD),
                    String::from("test today"),
                )),
                date,
            )
        }

        assets.calculate_balance(iso::USD);
        assert_eq!(assets.balances.last().unwrap().value(), Money::from_major(-30, iso::USD));
        assets.push_balance(Balance::new(Money::from_major(25, iso::USD), Utc::now()));
        assert_eq!(assets.balances.last().unwrap().value(), Money::from_major(25, iso::USD));

        revenue.calculate_balance(iso::USD);
        assert_eq!(revenue.balances.last().unwrap().value(), Money::from_major(30, iso::USD));
        revenue.add_transaction(
            Transaction::Debit(Entry::new(
                Money::from_major(10, iso::USD),
                String::from("removing"),
            )),
            Utc::now(),
        );
        assert_eq!(revenue.balances.len(), 1);
        revenue.calculate_balance(iso::USD);
        assert_eq!(revenue.balances.len(), 2);
        assert_eq!(revenue.balances.first().unwrap().value(), Money::from_major(30, iso::USD));
        assert_eq!(revenue.balances.last().unwrap().value(), Money::from_major(20, iso::USD));
    }
}
