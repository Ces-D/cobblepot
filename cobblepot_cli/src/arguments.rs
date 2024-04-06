use std::str::FromStr;

use crate::utils::parse_money;
use clap::{Arg, ArgMatches};
use cobblepot_accounting::account::AccountCode;
use cobblepot_core::error::CobblepotError;
use rusty_money::iso::USD;
use rusty_money::{iso::Currency, Money};

pub fn memo() -> Arg {
    Arg::new("memo")
        .short('m')
        .help("Note providing further details about this transaction")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .action(clap::ArgAction::Set)
}
pub fn parse_memo(matches: &ArgMatches) -> Result<&String, CobblepotError> {
    let memo: &String = matches
        .get_one("memo")
        .ok_or_else(|| CobblepotError::ParseValueError("Memo argument missing"))?;
    Ok(memo)
}

pub fn amount() -> Arg {
    Arg::new("amount")
        .short('a')
        .help("Monetary amount of this transaction")
        .value_parser(clap::builder::ValueParser::new(parse_money))
        .action(clap::ArgAction::Set)
}
pub fn parse_amount(matches: &ArgMatches) -> Result<Money<'static, Currency>, CobblepotError> {
    let amount: &String = matches
        .get_one("amount")
        .ok_or_else(|| CobblepotError::ParseValueError("Amount argument missing"))?;
    Ok(Money::from_str(amount.as_str(), USD)
        .map_err(|_| CobblepotError::ParseValueError("Amount argument parse into money error")))?
}

pub fn account_code() -> Arg {
    Arg::new("account_code")
        .short('c')
        .help("Account code for this transaction")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .action(clap::ArgAction::Set)
}
pub fn parse_account_code(matches: &ArgMatches) -> Result<AccountCode, CobblepotError> {
    let account_code: &String = matches
        .get_one("account_code")
        .ok_or_else(|| CobblepotError::ParseValueError("Account code argument missing"))?;
    Ok(AccountCode::from_str(account_code.as_str()))?
}
