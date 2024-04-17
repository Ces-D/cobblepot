use std::io::{BufReader, BufWriter};

use crate::vault;
use cobblepot_accounting::journal_entry;

// create the functions that will create or append to the journal.csv
pub struct Journal {
    config: vault::VaultConfig,
}

impl Journal {
    pub fn new(config: vault::VaultConfig) -> Self {
        Journal { config }
    }

    fn open_location(&self, read: bool, write: bool) -> std::fs::File {
        let path = self.config.location_as_pathbuf().join("journal.csv");
        std::fs::OpenOptions::new()
            .read(read)
            .append(write)
            .create(true)
            .open(path)
            .expect("Unable to open the file")
    }

    pub fn create_entry(
        &self,
        entry: &journal_entry::JournalEntry,
    ) -> Option<journal_entry::JournalEntry> {
        let exists = self.find_entry(entry.entry_id()).is_some();
        if exists {
            return None;
        }
        let writer = BufWriter::new(self.open_location(false, true));
        let mut writer = csv::Writer::from_writer(writer);
        match writer.serialize(entry) {
            Ok(_) => Some(entry.clone()),
            Err(e) => {
                eprintln!("Error writing to file: {}", e);
                return None;
            },
        }
    }

    pub fn find_entry(
        &self,
        entry_id: journal_entry::EntryId,
    ) -> Option<journal_entry::JournalEntry> {
        let reader = BufReader::new(self.open_location(true, false));
        let mut reader = csv::Reader::from_reader(reader);
        let mut journal = reader.deserialize::<journal_entry::JournalEntry>();
        let entry = journal.find(|row| row.as_ref().unwrap().entry_id() == entry_id);
        if entry.is_some() {
            return Some(entry.unwrap().expect("Unable to deserialize entry").clone());
        }
        return None;
    }
}
