use cobblepot_core::error::CobblepotError;
use rusty_money::iso::USD;
use rusty_money::MoneyError;
use rusty_money::{iso::Currency, Money};

pub fn parse_money(input: &str) -> Result<Money<'static, Currency>, MoneyError> {
    Money::from_str(input, USD)
}
