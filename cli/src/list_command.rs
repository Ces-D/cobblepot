use crate::{
    alert, list_command,
    logger::table::{ColumnConfig, Table},
    shared::AccountType,
    success,
};
use clap::{Parser, Subcommand, ValueEnum};
use diesel::SqliteConnection;

mod dto;

#[derive(Debug, Subcommand)]
enum ListCommand {
    #[clap(about = "List your accounts")]
    Accounts {
        #[clap(short, long, help = "Include only accounts of type")]
        account_type: Option<AccountType>,
        #[clap(short, long, help = "Include only accounts opened by")]
        opened_by: Option<chrono::NaiveDateTime>,
    },
    #[clap(about = "List your account balances")]
    Balances {
        #[clap(short, long, help = "Include only balances belonging to this account")]
        account_id: Option<i32>,
        #[clap(short, long, help = "Include only balances entered by")]
        entered_by: Option<chrono::NaiveDateTime>,
    },
    #[clap(about = "List your financial market instruments")]
    MarketInsruments {
        #[clap(short, long, help = "Include only instruments belonging to this account")]
        account_id: Option<i32>,
    },
    #[clap(about = "List your transactions that are recurring")]
    RecurringTransactions {
        #[clap(short, long, help = "Include only transactions belonging to this account")]
        account_id: Option<i32>,
        #[clap(short, long, help = "Include closed transactions")]
        closed: Option<bool>,
    },
}

#[derive(Debug, Parser)]
pub struct ListArgs {
    #[clap(subcommand)]
    pub command: ListCommand,
}

fn format_accounts_message(
    account_type: Option<AccountType>,
    opened_by: Option<chrono::NaiveDateTime>,
) -> String {
    let mut parts = vec![];

    if let Some(acc_type) = account_type {
        parts.push(format!("type: {}", acc_type.to_possible_value().unwrap().get_name()));
    }

    if let Some(date) = opened_by {
        parts.push(format!("opened by: {}", date.format("%Y-%m-%d")));
    }

    if parts.is_empty() {
        "Listed all accounts".to_string()
    } else {
        format!("Listed accounts filtered by {}", parts.join(", "))
    }
}

fn format_balances_message(
    account_id: Option<i32>,
    entered_by: Option<chrono::NaiveDateTime>,
) -> String {
    let mut parts = vec![];

    if let Some(id) = account_id {
        parts.push(format!("account ID: {}", id));
    }

    if let Some(date) = entered_by {
        parts.push(format!("entered by: {}", date.format("%Y-%m-%d")));
    }

    if parts.is_empty() {
        "Listed all balances".to_string()
    } else {
        format!("Listed balances filtered by {}", parts.join(", "))
    }
}

fn format_market_instruments_message(account_id: Option<i32>) -> String {
    if let Some(id) = account_id {
        format!("Listed market instruments filtered by account ID: {}", id)
    } else {
        "Listed all market instruments".to_string()
    }
}

fn format_recurring_transactions_message(account_id: Option<i32>, closed: Option<bool>) -> String {
    let mut parts = vec![];

    if let Some(id) = account_id {
        parts.push(format!("account ID: {}", id));
    }

    if let Some(is_closed) = closed {
        parts.push(format!("closed: {}", is_closed));
    }

    if parts.is_empty() {
        "Listed all recurring transactions".to_string()
    } else {
        format!("Listed recurring transactions filtered by {}", parts.join(", "))
    }
}

pub fn handle_list_command(args: ListArgs, conn: SqliteConnection) {
    match args.command {
        ListCommand::Accounts {
            account_type,
            opened_by,
        } => match list_command::dto::get_filtered_accounts(conn, account_type, opened_by) {
            Ok(r) => {
                success!("{}", format_accounts_message(account_type, opened_by));
                let mut table = Table::new(vec![
                    ColumnConfig::new("ID").min_width(5),
                    ColumnConfig::new("Name").max_width(30),
                    ColumnConfig::new("Description"),
                    ColumnConfig::new("Opened-On"),
                ]);
                for account in r {
                    table.push_row(vec![
                        account.id.to_string().as_str(),
                        &account.name,
                        &account.description.unwrap_or_else(|| "".to_string()),
                        &account.opened_on.format("%Y-%m-%d").to_string(),
                    ]);
                }
                table.display();
            }
            Err(e) => {
                alert!("Failed to list accounts");
                log::error!("List Accounts: {}", e)
            }
        },
        ListCommand::Balances {
            account_id,
            entered_by,
        } => {
            success!("{}", format_balances_message(account_id, entered_by));
            todo!();
        }
        ListCommand::MarketInsruments {
            account_id,
        } => {
            success!("{}", format_market_instruments_message(account_id));
            todo!();
        }
        ListCommand::RecurringTransactions {
            account_id,
            closed,
        } => {
            success!("{}", format_recurring_transactions_message(account_id, closed));
            todo!();
        }
    }
}
