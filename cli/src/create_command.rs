mod dto;

use crate::{
    alert,
    shared::{AccountType, InstrumentType},
    success,
};
use clap::{Parser, Subcommand};
use diesel::SqliteConnection;

#[derive(Debug, Subcommand)]
pub enum CreateCommand {
    #[clap(about = "Create an account")]
    Account {
        #[clap(help = "Name of the account")]
        name: String,
        #[clap(short, long)]
        description: Option<String>,
        #[clap(short, long)]
        owner: Option<String>,
        #[clap(short, long)]
        account_type: Option<AccountType>,
        #[clap(long, value_parser = crate::shared::parse_date, help = "Date account was opened (format: YYYY-MM-DD)")]
        opened_on: Option<chrono::NaiveDateTime>,
    },
    #[clap(about = "Create an account balance")]
    Balances {
        #[clap(help = "Id of the account this belongs to")]
        account_id: i32,
        #[clap(help = "Current balance amount")]
        amount: f32,
        #[clap(short, long)]
        memo: Option<String>,
        #[clap(short, long, value_parser = crate::shared::parse_date, help = "Date balance was entered (format: YYYY-MM-DD)")]
        entered_on: Option<chrono::NaiveDateTime>,
    },
    #[clap(about = "Create a financial market instruments")]
    MarketInsruments {
        #[clap(help = "Ticker of the stock")]
        ticker: String,
        name: String,
        #[clap(help = "Id of the account this belongs to")]
        account_id: i32,
        #[clap(short, long, default_value = "stock")]
        instrument_type: Option<InstrumentType>,
        #[clap(short, long, help = "")]
        market: Option<String>,
        #[clap(long, value_parser = crate::shared::parse_date, help = "Date instrument was opened (format: YYYY-MM-DD)")]
        opened_on: Option<chrono::NaiveDateTime>,
    },
    #[clap(about = "Create a recurring transaction")]
    RecurringTransactions {
        name: String,
        rrule: String,
        amount: f32,
        #[clap(value_parser = crate::shared::parse_date, help = "Start date for recurring transaction (format: YYYY-MM-DD)")]
        start_date: chrono::NaiveDateTime,
        description: Option<String>,
        account_type: Option<AccountType>,
    },
}

#[derive(Debug, Parser)]
pub struct CreateArgs {
    #[clap(subcommand)]
    pub command: CreateCommand,
}

pub fn handle_create_command(args: CreateArgs, conn: SqliteConnection) {
    match args.command {
        CreateCommand::Account {
            name,
            description,
            owner,
            account_type,
            opened_on,
        } => {
            match dto::create_new_account(conn, name, description, owner, account_type, opened_on) {
                Ok(res) => success!("Created new account: {} - {}", res.0, res.1),
                Err(e) => {
                    alert!("Failed to create new account");
                    log::error!("Create Account: {}", e);
                }
            };
        }
        CreateCommand::Balances {
            account_id,
            amount,
            memo,
            entered_on,
        } => match dto::create_new_balance(conn, account_id, amount, memo, entered_on) {
            Ok(res) => success!("Created new balance: {} - {}", res.0, res.1),
            Err(e) => {
                alert!("Failed to create new balance");
                log::error!("Create Balance: {}", e);
            }
        },
        CreateCommand::MarketInsruments {
            ticker,
            name,
            account_id,
            instrument_type,
            market,
            opened_on,
        } => todo!(),
        CreateCommand::RecurringTransactions {
            name,
            rrule,
            amount,
            start_date,
            description,
            account_type,
        } => todo!(),
    }
}
