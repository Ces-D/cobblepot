use crate::currency::Amount;
use chrono::Utc;

#[derive(Clone, Copy)]
pub struct Summary {
    pub debits: Amount,
    pub credits: Amount,
}

pub struct Transfer {
    id: String,
    pub amount: Summary,
    pub timestamp: chrono::DateTime<Utc>,
    pub memo: String,
}

impl Transfer {
    pub fn new(
        id: String,
        amount: Summary,
        timestamp: Option<chrono::DateTime<Utc>>,
        memo: String,
    ) -> Transfer {
        Transfer { id, amount, timestamp: timestamp.unwrap_or(chrono::offset::Utc::now()), memo }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }
}

pub struct Balance {
    pub timestamp: chrono::DateTime<Utc>,
    pub summary: Summary,
}
