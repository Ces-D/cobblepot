mod dto;

use crate::{alert, shared::AccountType, success};
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
    Balance {
        #[clap(help = "Id of the account this belongs to")]
        account_id: i32,
        #[clap(help = "Current balance amount")]
        amount: f32,
        #[clap(short, long)]
        memo: Option<String>,
        #[clap(short, long, value_parser = crate::shared::parse_date, help = "Date balance was entered (format: YYYY-MM-DD)")]
        entered_on: Option<chrono::NaiveDateTime>,
    },
    #[clap(about = "Create a budget")]
    Budget {
        #[clap(help = "Name of the budget")]
        name: String,
        #[clap(short, long, help = "Description of the budget")]
        description: Option<String>,
        #[clap(short, long, help = "Expected dollar limit")]
        anticipated_amount: f32,
        #[clap(short='s', long, value_parser = crate::shared::parse_date, help = "Date budget recurrence starts (format: YYYY-MM-DD)")]
        r_start: Option<chrono::NaiveDateTime>,
        #[clap(
            short,
            long,
            help = "Recurrence rule signaling repeating budget. Must pair with r_start"
        )]
        r_rule: Option<String>,
    },
    #[clap(about = "Create a budget line item")]
    BudgetItem {
        #[clap(help = "Id of the budget")]
        budget_id: i32,
        #[clap(help = "Name of the line item")]
        name: String,
        #[clap(short, long, help = "Description of the line item")]
        description: Option<String>,
        #[clap(help = "Value of this item")]
        amount: f32,
        #[clap(short='s', long, value_parser = crate::shared::parse_date, help = "Date budget recurrence starts (format: YYYY-MM-DD)")]
        r_start: Option<chrono::NaiveDateTime>,
        #[clap(
            short,
            long,
            help = "Recurrence rule signaling repeating budget item. Must pair with r_start"
        )]
        r_rule: Option<String>,
    },
    #[clap(about = "Create Additional metadata for a budget line item")]
    BudgetItemMeta {
        #[clap(help = "Id of the budget line item")]
        budget_item_id: i32,
        #[clap(help = "Id of the account that financially supports this budget item")]
        account_id: i32,
        #[clap(
            help = "Percentage of the budget item amount that is supported by the account (0-100)"
        )]
        percentage: Option<i32>,
    },
    // TODO: add this feature again
    // #[clap(about = "Create a financial market instruments")]
    // MarketInsruments {
    //     #[clap(help = "Ticker of the stock")]
    //     ticker: String,
    //     name: String,
    //     #[clap(help = "Id of the account this belongs to")]
    //     account_id: i32,
    //     #[clap(short, long, default_value = "stock")]
    //     instrument_type: Option<InstrumentType>,
    //     #[clap(short, long, help = "")]
    //     market: Option<String>,
    //     #[clap(long, value_parser = crate::shared::parse_date, help = "Date instrument was opened (format: YYYY-MM-DD)")]
    //     opened_on: Option<chrono::NaiveDateTime>,
    // },
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
        CreateCommand::Balance {
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
        CreateCommand::Budget {
            name,
            description,
            anticipated_amount,
            r_start,
            r_rule,
        } => {
            let recurrence_rule = r_start.zip(r_rule);
            if recurrence_rule.is_none() {
                alert!("Missing either r_start or r_rule. Skipping recurrence.");
            }
            match dto::create_new_budget(
                conn,
                name,
                description,
                anticipated_amount,
                recurrence_rule,
            ) {
                Ok(res) => success!("Created new budget: {} - {}", res.0, res.1),
                Err(e) => {
                    alert!("Failed to create new budget");
                    log::error!("Create Budget: {}", e);
                }
            }
        }
        CreateCommand::BudgetItem {
            budget_id,
            name,
            description,
            amount,
            r_start,
            r_rule,
        } => {
            let recurrence_rule = r_start.zip(r_rule);
            if recurrence_rule.is_none() {
                alert!("Missing either r_start or r_rule. Skipping recurrence.");
            }
            match dto::create_new_budget_item(
                conn,
                name,
                description,
                amount,
                recurrence_rule,
                budget_id,
            ) {
                Ok(res) => success!("Created new budget item: {} - {}", res.0, res.1),
                Err(e) => {
                    alert!("Failed to create new budget item");
                    log::error!("Create Budget Item: {}", e);
                }
            }
        }
        CreateCommand::BudgetItemMeta {
            budget_item_id,
            account_id,
            percentage,
        } => match dto::create_new_budget_item_account_meta(
            conn,
            budget_item_id,
            account_id,
            percentage,
        ) {
            Ok(res) => success!("Created {} new budget line item to account relationship", res),
            Err(e) => {
                alert!("Failed to create new budget item");
                log::error!("Create Budget Item: {}", e);
            }
        },
    }
}
