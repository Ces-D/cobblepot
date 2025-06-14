use chrono::{DateTime, Utc};
use cli_docs_macro::CliDocs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, CliDocs)]
pub struct JSONApplyRecurringTransaction {
    #[cli_docs(description = "ID of the recurring transaction to apply")]
    pub id: i32,
    #[cli_docs(description = "Start date of the application")]
    pub from: DateTime<Utc>,
    #[cli_docs(description = "End date of the application")]
    pub to: Option<DateTime<Utc>>,
}
