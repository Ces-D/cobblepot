use crate::code::AccountCode;
use crate::currency::{Amount, ExchangeRate};

#[derive(Debug, Clone, Copy)]
pub struct AccountBalance {
    pub debit_balance: Amount,
    pub credit_balance: Amount,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Default for AccountBalance {
    fn default() -> AccountBalance {
        AccountBalance {
            debit_balance: Amount::new(ExchangeRate::default()),
            credit_balance: Amount::new(ExchangeRate::default()),
            timestamp: chrono::offset::Utc::now(),
        }
    }
}

pub struct Account {
    id: String,
    pub created: chrono::DateTime<chrono::Utc>,
    code: AccountCode,
    pub balance: AccountBalance,
    pub maintain_history: bool,
}

impl Account {
    pub fn new(
        id: String,
        created: Option<chrono::DateTime<chrono::Utc>>,
        code: AccountCode,
        balance: Option<AccountBalance>,
        maintain_history: bool,
    ) -> Account {
        let created = created.unwrap_or(chrono::offset::Utc::now());
        let balance = balance.unwrap_or(AccountBalance::default());

        Account { id, created, code, balance, maintain_history }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn code(&self) -> AccountCode {
        self.code
    }
}
