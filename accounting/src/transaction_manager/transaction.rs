use super::entry::Entry;
use crate::account_manager::account_code::AccountCode;

pub enum Transaction {
    Credit(Entry, AccountCode),
    Debit(Entry, AccountCode),
}
