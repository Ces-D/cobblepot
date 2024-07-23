use chrono::Utc;

use crate::currency::iso::{Currency, USD};
use crate::currency::Amount;

// TODO: how to incorporate a transfer aka a journal entry
pub struct Transfer {
    id: String,
    pub amount: Amount,
    pub original_currency: Currency,
    pub timestamp: chrono::DateTime<Utc>,
    pub memo: String,
}

impl Transfer {
    pub fn new(
        id: String,
        amount: Amount,
        original_currency: Option<Currency>,
        timestamp: Option<chrono::DateTime<Utc>>,
        memo: String,
    ) -> Transfer {
        Transfer {
            id,
            amount,
            original_currency: original_currency.unwrap_or(*USD),
            timestamp: timestamp.unwrap_or(chrono::offset::Utc::now()),
            memo,
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }
}
