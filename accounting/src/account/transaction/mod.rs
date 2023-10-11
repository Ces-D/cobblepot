use crate::account::transaction::entry::Entry;
pub mod entry;

#[derive(PartialEq, Debug)]
pub enum Transaction {
    Credit(Entry),
    Debit(Entry),
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusty_money::iso;
    use rusty_money::Money;

    #[test]
    fn test_transaction_credit() {
        let amount = Money::from_str("100.00", iso::USD).unwrap();
        let memo = String::from("Test credit");
        let entry = Entry::new(amount, memo);
        let transaction = Transaction::Credit(entry);
        match transaction {
            Transaction::Credit(entry) => {
                assert_eq!(Money::from_str("100.00", iso::USD).unwrap(), entry.read_amount());
                assert_eq!("Test credit", entry.read_memo());
            },
            _ => panic!("Transaction is not a credit"),
        }
    }

    #[test]
    fn test_transaction_debit() {
        let amount = Money::from_str("100.00", iso::USD).unwrap();
        let memo = String::from("Test debit");
        let entry = Entry::new(amount, memo);
        let transaction = Transaction::Debit(entry);
        match transaction {
            Transaction::Debit(entry) => {
                assert_eq!(Money::from_str("100.00", iso::USD).unwrap(), entry.read_amount());
                assert_eq!("Test debit", entry.read_memo());
            },
            _ => panic!("Transaction is not a debit"),
        }
    }
}
