use std::str::FromStr;

use clap::{Arg, ArgMatches};
use cobblepot_accounting::account::{AccountCode, AccountType};
use cobblepot_accounting::money::Money;
use cobblepot_core::error::CobblepotError;

use crate::utils::MoneyParser;

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
        .value_parser(clap::builder::ValueParser::new(MoneyParser))
        .action(clap::ArgAction::Set)
}
pub fn parse_amount(matches: &ArgMatches) -> Result<Money, CobblepotError> {
    let amount: &String = matches
        .get_one("amount")
        .ok_or_else(|| CobblepotError::ParseValueError("Amount argument missing"))?;
    let money = Money::from_str(amount.as_str())
        .map_err(|_| CobblepotError::ParseValueError("Amount argument parse into money error"))?;
    Ok(money)
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

pub fn account_name() -> Arg {
    Arg::new("account_name")
        .short('n')
        .help("Account name")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .action(clap::ArgAction::Set)
}
pub fn parse_account_name(matches: &ArgMatches) -> Result<&String, CobblepotError> {
    let account_name: &String = matches
        .get_one("account_name")
        .ok_or_else(|| CobblepotError::ParseValueError("Account name missing"))?;
    Ok(account_name)
}

pub fn account_description() -> Arg {
    Arg::new("account_description")
        .short('d')
        .help("Description of the account")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .action(clap::ArgAction::Set)
}
pub fn parse_account_description(matches: &ArgMatches) -> Result<&String, CobblepotError> {
    let account_description: &String = matches
        .get_one("account_description")
        .ok_or_else(|| CobblepotError::ParseValueError("Account description missing"))?;
    Ok(account_description)
}

pub fn account_type() -> Arg {
    Arg::new("account_type")
        .short('t')
        .help("Type of this account")
        .value_parser(clap::builder::EnumValueParser::<AccountType>::new())
}
pub fn parse_account_type(matches: &ArgMatches) -> Result<&AccountType, CobblepotError> {
    let account_type: &AccountType = matches
        .get_one("account_type")
        .ok_or_else(|| CobblepotError::ParseValueError("Account type missing"))?;
    Ok(account_type)
}
