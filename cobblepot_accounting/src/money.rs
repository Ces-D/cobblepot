use cobblepot_core::error::CobblepotError;
use rust_decimal::prelude::FromStr;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
/// Always in USD
pub struct Money {
    #[serde(with = "rust_decimal::serde::str")]
    amount: Decimal,
}

impl Money {
    pub fn from_major(major: i64) -> Money {
        Money { amount: Decimal::new(major, 0) }
    }

    pub fn from_minor(minor: i64) -> Money {
        Money { amount: Decimal::new(minor, 2) }
    }

    pub fn from_str(amount: &str) -> Result<Money, CobblepotError> {
        let amount = Decimal::from_str(amount).map_err(|_| CobblepotError::InvalidMoneyFormat)?;

        Ok(Money { amount })
    }

    pub fn amount(&self) -> Decimal {
        self.amount
    }
}

impl Add for Money {
    type Output = Money;
    fn add(self, other: Money) -> Money {
        Money { amount: self.amount + other.amount }
    }
}

impl AddAssign for Money {
    fn add_assign(&mut self, other: Self) {
        *self = Self { amount: self.amount + other.amount };
    }
}

impl Sub for Money {
    type Output = Money;
    fn sub(self, other: Money) -> Money {
        Money { amount: self.amount - other.amount }
    }
}

impl SubAssign for Money {
    fn sub_assign(&mut self, other: Self) {
        *self = Self { amount: self.amount - other.amount };
    }
}

impl Neg for Money {
    type Output = Money;

    fn neg(self) -> Self::Output {
        Money { amount: -self.amount }
    }
}
