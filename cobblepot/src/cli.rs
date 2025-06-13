use clap::{Arg, Command, crate_description, crate_name, crate_version};
use diesel::SqliteConnection;
use strum::{AsRefStr, EnumString};

use crate::{
    account::model::{CliCloseAccount, CliOpenAccount, CliUpdateAccount},
    balance::model::{CliOpenBalance, CliUpdateBalance},
    recurring_transation::model::CliOpenRecurringTransaction,
    shared::{CobblepotError, CobblepotResult},
};

#[derive(AsRefStr, EnumString, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum CobblepotCommand {
    Open,
    Update,
    Close,
    Apply,
    Account,
    Balance,
    Recurring,
    Report,
}

pub fn command() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .long_about(None)
        .subcommand_required(true)
        .subcommand(
            Command::new(CobblepotCommand::Open.as_ref())
                .about("Open a new entity")
                .subcommand_required(true)
                .subcommands([
                    Command::new(CobblepotCommand::Account.as_ref())
                        .about("Open a new account")
                        .after_help(CliOpenAccount::example())
                        .arg(
                            Arg::new("data")
                                .help("JSON object containing open account data")
                                .long_help(CliOpenAccount::help())
                                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                                .required(true),
                        ),
                    Command::new(CobblepotCommand::Balance.as_ref())
                        .about("Open a new balance")
                        .after_help(CliOpenBalance::example())
                        .arg(
                            Arg::new("data")
                                .help("JSON object containing open balance data")
                                .long_help(CliOpenBalance::help())
                                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                                .required(true),
                        ),
                    Command::new(CobblepotCommand::Recurring.as_ref())
                        .about("Open a new recurring entity")
                        .after_help(CliOpenRecurringTransaction::example())
                        .arg(
                            Arg::new("data")
                                .help("JSON object containing open recurring transaction data")
                                .long(CliOpenRecurringTransaction::help())
                                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                                .required(true),
                        ),
                    Command::new(CobblepotCommand::Report.as_ref()).about("Generate a report").arg(
                        Arg::new("data")
                            .help("JSON object containing open report data")
                            .value_parser(clap::builder::NonEmptyStringValueParser::new())
                            .required(true),
                    ),
                ]),
        )
        .subcommand(
            Command::new(CobblepotCommand::Update.as_ref())
                .about("Update an existing entity")
                .subcommand_required(true)
                .subcommands([
                    Command::new(CobblepotCommand::Account.as_ref())
                        .about("Update an existing account")
                        .after_help(CliUpdateAccount::example())
                        .arg(
                            Arg::new("data")
                                .help("JSON object containing update account data")
                                .long_help(CliUpdateAccount::help())
                                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                                .required(true),
                        ),
                    Command::new(CobblepotCommand::Balance.as_ref())
                        .about("Update an existing balance")
                        .after_help(CliUpdateBalance::example())
                        .arg(
                            Arg::new("data")
                                .help("JSON object containing update balance data")
                                .long_help(CliUpdateBalance::help())
                                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                                .required(true),
                        ),
                ]),
        )
        .subcommand(
            Command::new(CobblepotCommand::Close.as_ref())
                .about("Close an existing entity")
                .subcommand_required(true)
                .subcommands([
                    Command::new(CobblepotCommand::Account.as_ref())
                        .about("Close an existing account")
                        .after_help(CliCloseAccount::example())
                        .arg(
                            Arg::new("data")
                                .help("JSON object containing close account data")
                                .long_help(CliCloseAccount::help())
                                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                                .required(true),
                        ),
                    Command::new(CobblepotCommand::Recurring.as_ref())
                        .about("Close an existing recurring entity")
                        .arg(
                            Arg::new("data")
                                .help("JSON object containing close recurring transaction data")
                                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                                .required(true),
                        ),
                ]),
        )
        .subcommand(
            Command::new(CobblepotCommand::Apply.as_ref())
                .about("Apply an effect to balances")
                .subcommand_required(true)
                .subcommands([Command::new(CobblepotCommand::Recurring.as_ref())
                    .about("Apply the effect of a recurring transaction to its account")
                    .arg(
                        Arg::new("data")
                            .help("JSON object containing close recurring transaction data")
                            .value_parser(clap::builder::NonEmptyStringValueParser::new())
                            .required(true),
                    )]),
        )
}

pub fn handle(
    action: CobblepotCommand,
    noun: CobblepotCommand,
    connection: SqliteConnection,
    data: &String,
) -> CobblepotResult<String> {
    match action {
        CobblepotCommand::Open => match noun {
            CobblepotCommand::Account => {
                let open_account =
                    serde_json::from_str::<crate::account::model::CliOpenAccount>(&data)?;
                let new_account =
                    crate::account::service::insert_new_account(open_account, connection)?;
                let res = serde_json::to_string_pretty(&new_account)?;
                Ok(res)
            }
            CobblepotCommand::Balance => {
                let open_balance =
                    serde_json::from_str::<crate::balance::model::CliOpenBalance>(&data)?;
                let new_balance =
                    crate::balance::service::insert_new_balance(open_balance, connection)?;
                let res = serde_json::to_string_pretty(&new_balance)?;
                Ok(res)
            }
            CobblepotCommand::Recurring => {
                let open_recurring = serde_json::from_str::<
                    crate::recurring_transation::model::CliOpenRecurringTransaction,
                >(&data)?;
                let new_recurring =
                    crate::recurring_transation::service::insert_new_recurring_transaction(
                        open_recurring,
                        connection,
                    )?;
                let res = serde_json::to_string_pretty(&new_recurring)?;
                Ok(res)
            }
            CobblepotCommand::Report => {
                let open_report =
                    serde_json::from_str::<crate::report::model::CliOpenReport>(&data)?;
                match open_report.report_tye {
                    crate::shared::ReportType::BalanceSheet => {
                        let balance_sheet = crate::report::service::create_balance_sheet_report(
                            open_report,
                            connection,
                        )?;
                        let res = serde_json::to_string_pretty(&balance_sheet)?;
                        Ok(res)
                    }
                    crate::shared::ReportType::DeepDiveAccount => {
                        let deep_dive = crate::report::service::create_deep_dive_account_report(
                            open_report,
                            connection,
                        )?;
                        let res = serde_json::to_string_pretty(&deep_dive)?;
                        Ok(res)
                    }
                    crate::shared::ReportType::DeepDiveRecurring => todo!(),
                }
            }
            _ => Err(CobblepotError::CliCommandError(
                "Unsupported `open` command. Use --help for more instruction",
            )),
        },
        CobblepotCommand::Update => match noun {
            CobblepotCommand::Account => {
                let update_account =
                    serde_json::from_str::<crate::account::model::CliUpdateAccount>(&data)?;
                let updated_account =
                    crate::account::service::update_account(update_account, connection)?;
                let res = serde_json::to_string_pretty(&updated_account)?;
                Ok(res)
            }
            CobblepotCommand::Balance => {
                let update_balance =
                    serde_json::from_str::<crate::balance::model::CliUpdateBalance>(&data)?;
                let updated_balance =
                    crate::balance::service::update_balance(update_balance, connection)?;
                let res = serde_json::to_string_pretty(&updated_balance)?;
                Ok(res)
            }
            _ => Err(CobblepotError::CliCommandError(
                "Unsupported `update` command. Use --help for more instruction",
            )),
        },
        CobblepotCommand::Close => match noun {
            CobblepotCommand::Account => {
                let close_account =
                    serde_json::from_str::<crate::account::model::CliCloseAccount>(&data)?;
                let closed_count =
                    crate::account::service::close_account(close_account, connection)?;
                Ok(format!("Closed {} account(s)", closed_count))
            }
            CobblepotCommand::Recurring => {
                let close_recurring = serde_json::from_str::<
                    crate::recurring_transation::model::CliCloseRecurringTransaction,
                >(&data)?;
                let closed_count =
                    crate::recurring_transation::service::close_recurring_transaction(
                        close_recurring,
                        connection,
                    )?;
                Ok(format!("Closed {} recurring transaction(s)", closed_count))
            }
            _ => Err(CobblepotError::CliCommandError(
                "Unsupported `close` command. Use --help for more instruction",
            )),
        },
        _ => Err(CobblepotError::CliCommandError(
            "Unsupported action command. Use --help for more instruction",
        )),
    }
}
