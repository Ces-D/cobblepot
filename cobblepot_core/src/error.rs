#[derive(Debug)]
pub enum CobblepotError {
    InvalidMoneyFormat,
    AddJournalEntryCliError(&'static str),
    AddChartOfAccountsCliError(&'static str),
    ParseValueError(&'static str),
    VaultCreationError(&'static str),
}

impl std::fmt::Display for CobblepotError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CobblepotError::InvalidMoneyFormat => write!(f, "Invalid money format"),
            CobblepotError::AddJournalEntryCliError(e) => {
                write!(f, "Error adding journal entry: {}", e)
            },
            CobblepotError::AddChartOfAccountsCliError(e) => {
                write!(f, "Error adding journal entry: {}", e)
            },
            CobblepotError::ParseValueError(e) => write!(f, "Error parsing value: {}", e),
            CobblepotError::VaultCreationError(e) => write!(f, "Error creating vault: {}", e),
        }
    }
}

impl std::error::Error for CobblepotError {}
