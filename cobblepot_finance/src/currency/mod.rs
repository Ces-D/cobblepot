use std::ops::{Add, Div, Mul, Sub};

use iso::{Currency, USD};
use rust_decimal::Decimal;

pub mod iso;
mod locale;

pub struct ExchangeRate {
    unit_currency: Currency,
    /// Ex. MXN 20.5
    unit_amount: Decimal,
    /// Ex. USD 1 -> MXN 17.61
    /// Ex. USD 1 -> EUR 0.92
    usd_exchange_rate: Decimal,
}

impl Default for ExchangeRate {
    fn default() -> ExchangeRate {
        ExchangeRate {
            unit_currency: *USD,
            unit_amount: Decimal::ZERO,
            usd_exchange_rate: Decimal::ONE,
        }
    }
}

/// Always stored relative in USD
#[derive(Debug, Clone, Copy)]
pub struct Amount(Decimal);

impl Amount {
    pub fn new(exchange: ExchangeRate) -> Amount {
        if exchange.unit_currency.symbol() == USD.symbol() {
            Amount(exchange.unit_amount)
        } else {
            assert!(exchange.unit_amount != Decimal::ZERO, "Unit amount must be >1");
            assert!(exchange.usd_exchange_rate > Decimal::ZERO, "Exchange rate must be >0");

            let value_in_usd = exchange.unit_amount / exchange.usd_exchange_rate;
            Amount(value_in_usd)
        }
    }

    pub fn value_in_usd(&self) -> Decimal {
        self.0.clone()
    }
}

impl From<Decimal> for Amount {
    fn from(value: Decimal) -> Self {
        Amount::new(ExchangeRate { unit_amount: value, ..ExchangeRate::default() })
    }
}

impl Add for Amount {
    type Output = Amount;

    fn add(self, rhs: Self) -> Self::Output {
        Amount(self.value_in_usd() + rhs.value_in_usd())
    }
}

impl Sub for Amount {
    type Output = Amount;

    fn sub(self, rhs: Self) -> Self::Output {
        Amount(self.value_in_usd() - rhs.value_in_usd())
    }
}

impl Div for Amount {
    type Output = Amount;

    fn div(self, rhs: Self) -> Self::Output {
        Amount(self.value_in_usd() / rhs.value_in_usd())
    }
}

impl Mul for Amount {
    type Output = Amount;

    fn mul(self, rhs: Self) -> Self::Output {
        Amount(self.value_in_usd() * rhs.value_in_usd())
    }
}
