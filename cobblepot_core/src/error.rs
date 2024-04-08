pub enum CobblepotError {
    InvalidMoneyFormat,
    AddJournalEntryCliError(&'static str),
    ParseValueError(&'static str),
    VaultCreationError(&'static str),
}
