use crate::vault;
use cobblepot_accounting::journal_entry;

// TODO: import the journal entry struct serializing them
// create the functions that will create or append to the journal.csv
pub struct Journal {
    config: vault::VaultConfig,
}

impl Journal {
    pub fn new(config: vault::VaultConfig) -> Self {
        Journal { config }
    }

    pub fn create_entry(
        entry: &journal_entry::JournalEntry,
    ) -> Option<&journal_entry::JournalEntry> {
        // check if file exists else create it
        // append the entry to the file
        // should return a reference to the passed in entry
        todo!("Implement the append_entry function")
    }
}
