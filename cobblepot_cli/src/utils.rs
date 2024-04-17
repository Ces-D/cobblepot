use cobblepot_accounting::money::Money;

#[derive(Debug, Clone)]
pub struct MoneyParser;

impl clap::builder::TypedValueParser for MoneyParser {
    type Value = Money;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let amount = value.to_str().unwrap_or("");
        let money = Money::from_str(amount);
        match money {
            Ok(money) => return Ok(money),
            Err(_) => {
                let mut err =
                    clap::Error::new(clap::error::ErrorKind::ValueValidation).with_cmd(cmd);
                if let Some(arg) = arg {
                    err.insert(
                        clap::error::ContextKind::InvalidArg,
                        clap::error::ContextValue::String(arg.to_string()),
                    );
                }
                return Err(err);
            },
        }
    }
}
