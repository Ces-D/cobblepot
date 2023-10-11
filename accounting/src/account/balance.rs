use chrono::{DateTime, Utc};
use rusty_money::iso::Currency;
use rusty_money::Money;

use super::account_type::AccountType;
use super::transaction::Transaction;

enum TransactionAffect {
    Increase(Money<'static, Currency>),
    Decrease(Money<'static, Currency>),
}

pub struct Balance {
    calculated_on: DateTime<Utc>,
    value: Money<'static, Currency>,
}

impl Balance {
    pub fn new(amount: Money<'static, Currency>, calculated_on: DateTime<Utc>) -> Balance {
        Balance { calculated_on, value: amount }
    }

    pub fn value(&self) -> Money<'static, Currency> {
        self.value.clone()
    }
    pub fn calculated_on(&self) -> DateTime<Utc> {
        self.calculated_on
    }

    pub fn calculate_post_transaction(
        &mut self,
        account_type: &AccountType,
        transaction: &Transaction,
    ) {
        let affect = self.identify_transaction_affect(&account_type, &transaction);
        match affect {
            TransactionAffect::Increase(amount) => {
                self.value += amount;
            },
            TransactionAffect::Decrease(amount) => {
                self.value -= amount;
            },
        }
    }

    fn identify_transaction_affect(
        &self,
        account_type: &AccountType,
        transaction: &Transaction,
    ) -> TransactionAffect {
        match account_type {
            AccountType::Asset => match transaction {
                Transaction::Credit(entry) => TransactionAffect::Decrease(entry.read_amount()),
                Transaction::Debit(entry) => TransactionAffect::Increase(entry.read_amount()),
            },
            AccountType::Liability => match transaction {
                Transaction::Credit(entry) => TransactionAffect::Increase(entry.read_amount()),
                Transaction::Debit(entry) => TransactionAffect::Decrease(entry.read_amount()),
            },
            AccountType::Equity => match transaction {
                Transaction::Credit(entry) => TransactionAffect::Increase(entry.read_amount()),
                Transaction::Debit(entry) => TransactionAffect::Decrease(entry.read_amount()),
            },
            AccountType::Revenue => match transaction {
                Transaction::Credit(entry) => TransactionAffect::Increase(entry.read_amount()),
                Transaction::Debit(entry) => TransactionAffect::Decrease(entry.read_amount()),
            },
            AccountType::Expense => match transaction {
                Transaction::Credit(entry) => TransactionAffect::Decrease(entry.read_amount()),
                Transaction::Debit(entry) => TransactionAffect::Increase(entry.read_amount()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::transaction::entry::Entry;
    use rusty_money::iso;

    fn create_transactions() -> (Transaction, Transaction, Transaction, Transaction, Transaction) {
        (
            Transaction::Credit(Entry::new(
                Money::from_major(100, iso::USD),
                "100 USD credit".to_string(),
            )),
            Transaction::Debit(Entry::new(
                Money::from_major(100, iso::USD),
                "100 USD debit".to_string(),
            )),
            Transaction::Credit(Entry::new(
                Money::from_minor(100, iso::USD),
                "1 USD credit".to_string(),
            )),
            Transaction::Debit(Entry::new(
                Money::from_minor(100, iso::USD),
                "1 USD debit".to_string(),
            )),
            Transaction::Debit(Entry::new(
                Money::from_major(100, iso::USD),
                "1 USD debit".to_string(),
            )),
        )
    }

    #[test]
    fn test_asset_balance() {
        let account_type = AccountType::Asset;

        let (
            credit_transaction_major,
            debit_transaction_major,
            credit_transaction_minor,
            debit_transaction_minor,
            another_transaction,
        ) = create_transactions();

        let mut balance = Balance::new(Money::from_major(0, iso::USD), Utc::now());
        assert_eq!(balance.value(), Money::from_major(0, iso::USD));
        balance.calculate_post_transaction(&account_type, &debit_transaction_major);
        assert_eq!(balance.value(), Money::from_major(100, iso::USD));
        balance.calculate_post_transaction(&account_type, &credit_transaction_major);
        assert_eq!(balance.value(), Money::from_major(0, iso::USD));
        balance.calculate_post_transaction(&account_type, &debit_transaction_minor);
        assert_eq!(balance.value(), Money::from_major(1, iso::USD));
        balance.calculate_post_transaction(&account_type, &credit_transaction_minor);
        assert_eq!(balance.value(), Money::from_major(0, iso::USD));
        balance.calculate_post_transaction(&account_type, &another_transaction);
        assert_eq!(balance.value(), Money::from_major(100, iso::USD));
    }

    #[test]
    fn test_revenue_balance() {
        let account_type = AccountType::Revenue;

        let (
            credit_transaction_major,
            debit_transaction_major,
            credit_transaction_minor,
            debit_transaction_minor,
            another_transaction,
        ) = create_transactions();

        let mut balance = Balance::new(Money::from_major(0, iso::USD), Utc::now());
        assert_eq!(balance.value(), Money::from_major(0, iso::USD));
        balance.calculate_post_transaction(&account_type, &debit_transaction_major);
        assert_eq!(balance.value(), Money::from_major(-100, iso::USD));
        balance.calculate_post_transaction(&account_type, &credit_transaction_major);
        assert_eq!(balance.value(), Money::from_major(0, iso::USD));
        balance.calculate_post_transaction(&account_type, &debit_transaction_minor);
        assert_eq!(balance.value(), Money::from_major(-1, iso::USD));
        balance.calculate_post_transaction(&account_type, &credit_transaction_minor);
        assert_eq!(balance.value(), Money::from_major(0, iso::USD));
        balance.calculate_post_transaction(&account_type, &another_transaction);
        assert_eq!(balance.value(), Money::from_major(-100, iso::USD));
    }
}
