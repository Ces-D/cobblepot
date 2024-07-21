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
pub struct Amount(pub Decimal);

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
}
