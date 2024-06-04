use std::io::{BufReader, BufWriter};

use crate::vault;

use super::utils::deserialize_csv;

pub struct Journal<'a> {
    config: &'a vault::VaultConfig,
}

impl<'a> Journal<'a> {
    pub fn new(config: &vault::VaultConfig) -> Journal<'_> {
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
        entry: &cobblepot_accounting::journal::Entry,
    ) -> Option<cobblepot_accounting::journal::Entry> {
        let exists = self.find_entry(entry.entry_code()).is_some();
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
        entry_code: cobblepot_accounting::codes::EntryCode,
    ) -> Option<cobblepot_accounting::journal::Entry> {
        let reader = BufReader::new(self.open_location(true, false));
        let mut journal =
            deserialize_csv::<cobblepot_accounting::journal::Entry>(reader).into_iter();
        let entry = journal.find(|row| row.entry_code() == entry_code);
        if entry.is_some() {
            return Some(entry.clone())?;
        }
        return None;
    }
}
