use cobblepot_core::error::CobblepotError;
use rust_decimal::Decimal;
use rusty_money::iso::Currency;
use rusty_money::{FormattableCurrency, Money};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionVariant {
    /// Transaction on something you own or possess
    Asset,

    /// Transaction on something owed to a non owner
    Liability,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    variant: TransactionVariant,
    amount: Decimal,
    currency: String,
}

impl Transaction {
    pub fn new<T: FormattableCurrency>(
        variant: TransactionVariant,
        money: Money<T>,
    ) -> Transaction {
        Transaction {
            variant,
            amount: money.amount().clone(),
            currency: money.currency().code().to_string(),
        }
    }

    pub fn variant(&self) -> TransactionVariant {
        self.variant.clone()
    }

    pub fn amount(&self) -> Money<'_, Currency> {
        let currency = rusty_money::iso::find_by_num_code(self.currency.as_str())
            .expect("Incorrect currency code stored");
        Money::from_decimal(self.amount, currency)
    }

    pub fn add(&self, change: Transaction) -> Result<Transaction, CobblepotError> {
        if change.variant == self.variant {
            let updated = self.amount() + change.amount();
            Ok(Transaction::new(self.variant.clone(), updated))
        } else {
            Err(CobblepotError::InvalidMoneyFormat)
        }
    }
    pub fn subtract(&self, change: Transaction) -> Result<Transaction, CobblepotError> {
        if change.variant == self.variant {
            let updated = self.amount() - change.amount();
            Ok(Transaction::new(self.variant.clone(), updated))
        } else {
            Err(CobblepotError::InvalidMoneyFormat)
        }
    }
}
