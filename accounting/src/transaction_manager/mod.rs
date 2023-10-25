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

        match void_target {
            Transaction::Credit(entry, account_code) => {
                let voiding = Transaction::Debit(
                    Entry::new(entry.amount(), format!("Void transaction: {}", entry.memo())),
                    account_code.clone(),
                );
                self.add_transaction(voiding);
            },
            Transaction::Debit(entry, account_code) => {
                let voiding = Transaction::Credit(
                    Entry::new(entry.amount(), format!("Void transaction: {}", entry.memo())),
                    account_code.clone(),
                );
                self.add_transaction(voiding);
            },
        }
    }
}
