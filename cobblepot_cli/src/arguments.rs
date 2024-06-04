use clap::{Arg, ArgMatches};
use cobblepot_accounting::codes::AccountCode;
use cobblepot_accounting::transaction::TransactionVariant;
use cobblepot_core::error::CobblepotError;
use rusty_money::{iso, FormattableCurrency, Money};
use std::str::FromStr;

use crate::parsers::TransactionVariantParser;

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

pub fn transaction_variant() -> Arg {
    Arg::new("transaction_variant")
        .short('t')
        .help("Optional account variant this transaction targets")
        .value_parser(TransactionVariantParser::new())
        .default_value("asset")
}
pub fn parse_transaction_variant(
    matches: &ArgMatches,
) -> Result<TransactionVariant, CobblepotError> {
    let account_type: &TransactionVariant = matches
        .get_one("account_type")
        .ok_or_else(|| CobblepotError::ParseValueError("Account type missing"))?;
    Ok(account_type.clone())
}

pub fn currency() -> Arg {
    Arg::new("currency")
        .short('u')
        .help("Currency of this transaction")
        .value_parser(crate::parsers::CurrencyValueParser::new())
        .default_value("USD")
        .action(clap::ArgAction::Set)
}
pub fn parse_currency(matches: &ArgMatches) -> Result<&'static iso::Currency, CobblepotError> {
    let currency_code: &String = matches
        .get_one("currency")
        .ok_or_else(|| CobblepotError::ParseValueError("Currency argument missing"))?;
    Ok(iso::find_by_num_code(currency_code.as_str()).expect("Parser failed"))
}

pub fn amount() -> Arg {
    Arg::new("amount")
        .short('a')
        .help("Monetary amount of this transaction")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .action(clap::ArgAction::Set)
}
pub fn parse_amount<'a, T: FormattableCurrency>(
    matches: &ArgMatches,
    currency: &'static T,
) -> Result<Money<'static, T>, CobblepotError> {
    let amount: &String = matches
        .get_one("amount")
        .ok_or_else(|| CobblepotError::ParseValueError("Amount argument missing"))?;
    let money = Money::from_str(amount.as_str(), currency)
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
