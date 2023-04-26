use rusty_money::{FormattableCurrency, Money, Params, Position};

/// Money formatter default params.
pub fn money_formatter_params<T: FormattableCurrency>(money: &Money<T>) -> Params {
    Params {
        code: Some(money.currency().code()),
        positions: vec![Position::Sign, Position::Space, Position::Symbol, Position::Amount],
        ..Params::default()
    }
}
