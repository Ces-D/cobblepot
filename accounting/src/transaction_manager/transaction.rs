use super::entry::Entry;
use crate::account_manager::account_code::AccountCode;

// We are focused on single entry accounting, otherwise we would have more tightly coupled a
// transaction to contain both a credit and debit entry
pub enum Transaction {
    Credit(Entry, AccountCode),
    Debit(Entry, AccountCode),
}
