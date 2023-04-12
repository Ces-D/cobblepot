use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum Currency {
    USD,
}

pub struct CurrencyConfig {
    primary_currency: Currency,
}

impl CurrencyConfig {
    pub fn new(primary_currency: Option<Currency>) -> CurrencyConfig {
        CurrencyConfig { primary_currency: primary_currency.unwrap_or_else(|| Currency::USD) }
    }

    pub fn primary_currency(&self) -> &Currency {
        &self.primary_currency
    }
}
