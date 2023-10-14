use std::collections::HashMap;
mod entry;
mod transaction;

use self::{entry::Entry, transaction::Transaction};

use chrono::{DateTime, Utc};

/// AKA: Journal
/// **Journal** - Records _transactions_ in the order they occur
struct TransactionManager {
    journal: HashMap<DateTime<Utc>, Transaction>,
}

impl TransactionManager {
    pub fn new() -> TransactionManager {
        TransactionManager { journal: HashMap::new() }
    }

    /// Inserts a new transaction, checking if it already exists
    pub fn add_transaction(&mut self, transaction: Transaction) {
        if self.journal.contains_key(&Utc::now()) {
            panic!("Transaction already exists");
        }
        self.journal.insert(Utc::now(), transaction);
    }

    pub fn get_transaction(&self, date: DateTime<Utc>) -> Option<&Transaction> {
        self.journal.get(&date)
    }

    /// Voids a transaction by creating a new opposite transaction to the original
    pub fn void_transaction(&mut self, date: DateTime<Utc>) {
        let void_target = self.get_transaction(date).expect("Transaction does not exist");

        let voiding = Transaction::new(
            (
                Entry::new(
                    void_target.debit.0.amount(),
                    format!("Voiding transaction: {}", void_target.debit.0.memo()),
                ),
                void_target.debit.1.clone(),
            ),
            (
                Entry::new(
                    void_target.credit.0.amount(),
                    format!("Voiding transaction: {}", void_target.credit.0.memo()),
                ),
                void_target.credit.1.clone(),
            ),
        );

        self.add_transaction(voiding);
    }
}

#[cfg(test)]
mod transaction_manager_tests {
    use super::*;
    use crate::account_manager::{account_code::AccountCode, account_type::AccountType};
    use rusty_money::{iso::USD, Money};

    #[test]
    fn add_transaction() {
        // TODO add a module for rules. Transaction rules, account rules, balance rules
        let mut tm = TransactionManager::new();
        let t = Transaction::new(
            (
                Entry::new(Money::from_major(100, USD), "Test".to_string()),
                AccountCode::new(AccountType::Asset, 0),
            ),
            (
                Entry::new(Money::from_major(100, USD), "Test".to_string()),
                AccountCode::new(AccountType::Asset, 0),
            ),
        );
        tm.add_transaction(t);
    }
}

//  TODO: resolve these conflicts
