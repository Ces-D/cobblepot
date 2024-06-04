use clap::error::ErrorKind;
use cobblepot_accounting::transaction::TransactionVariant;
use rusty_money::iso;

#[derive(Clone)]
pub struct CurrencyValueParser {}
impl CurrencyValueParser {
    pub fn new() -> CurrencyValueParser {
        CurrencyValueParser {}
    }
}
impl clap::builder::TypedValueParser for CurrencyValueParser {
    type Value = String;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let code = value.to_str().ok_or(clap::Error::new(ErrorKind::ValueValidation))?;
        match iso::find_by_num_code(code) {
            Some(currency) => Ok(currency.to_string()),
            None => Err(clap::Error::new(ErrorKind::ValueValidation)),
        }
    }
}

#[derive(Clone)]
pub struct TransactionVariantParser {}

impl TransactionVariantParser {
    pub fn new() -> TransactionVariantParser {
        TransactionVariantParser {}
    }
}
impl clap::builder::TypedValueParser for TransactionVariantParser {
    type Value = TransactionVariant;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let inner = clap::builder::PossibleValuesParser::new(["asset", "liability"]);
        let val = inner.parse_ref(cmd, arg, value)?;

        if val == "asset" {
            Ok(TransactionVariant::Asset)
        } else if val == "liability" {
            Ok(TransactionVariant::Liability)
        } else {
            Err(clap::Error::new(ErrorKind::InvalidValue))
        }
    }
}
