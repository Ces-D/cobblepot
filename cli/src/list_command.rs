use crate::{
    alert,
    logger::table::{ColumnConfig, Table},
    shared::{AccountType, format_money_usd},
    success,
};
use clap::{Parser, Subcommand, ValueEnum};
use diesel::SqliteConnection;
use rrule::Tz;

mod dto;

#[derive(Debug, Subcommand)]
pub enum ListCommand {
    #[clap(about = "List your accounts")]
    Accounts {
        #[clap(short, long, help = "Include only accounts of type")]
        account_type: Option<AccountType>,
        #[clap(short, long, value_parser = crate::shared::parse_date, help = "Include only accounts opened after this date (format: YYYY-MM-DD)")]
        opened_by: Option<chrono::NaiveDateTime>,
    },
    #[clap(about = "List your account balances")]
    Balances {
        #[clap(short, long, help = "Include only balances belonging to this account")]
        account_id: Option<i32>,
        #[clap(short, long, value_parser = crate::shared::parse_date, help = "Include only balances entered after this date (format: YYYY-MM-DD)")]
        entered_by: Option<chrono::NaiveDateTime>,
    },
    #[clap(about = "List your budgets")]
    Budgets,
    // #[clap(about = "List your financial market instruments")]
    // MarketInstruments {
    //     #[clap(short, long, help = "Include only instruments belonging to this account")]
    //     account_id: Option<i32>,
    // },
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

fn format_budgets_message() -> String {
    format!("Listed all budgets")
}

pub fn handle_list_command(args: ListArgs, conn: SqliteConnection) {
    match args.command {
        ListCommand::Accounts {
            account_type,
            opened_by,
        } => match dto::get_filtered_accounts(conn, account_type, opened_by) {
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
                        &account.opened_on.inner().format("%Y-%m-%d").to_string(),
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
        } => match dto::get_filtered_balances(conn, account_id, entered_by) {
            Ok(r) => {
                success!("{}", format_balances_message(account_id, entered_by));
                let mut table = Table::new(vec![
                    ColumnConfig::new("ID").max_width(5),
                    ColumnConfig::new("Amount").max_width(30),
                    ColumnConfig::new("Memo"),
                    ColumnConfig::new("Entered On"),
                ]);
                for balance in r {
                    table.push_row(vec![
                        balance.id.to_string().as_str(),
                        &format_money_usd(balance.amount),
                        &balance.memo,
                        &balance.entered_on.inner().format("%Y-%m-%d").to_string(),
                    ]);
                }
                table.display();
            }
            Err(e) => {
                alert!("Failed to list balances");
                log::error!("List Balances: {}", e)
            }
        },
        ListCommand::Budgets => match dto::get_filtered_budgets(conn) {
            Ok(r) => {
                success!("{}", format_budgets_message());
                let mut table = Table::new(vec![
                    ColumnConfig::new("ID").max_width(5),
                    ColumnConfig::new("Name"),
                    ColumnConfig::new("Description").max_width(30),
                    ColumnConfig::new("Anticipated Amount"),
                    ColumnConfig::new("DT Start"),
                    ColumnConfig::new("DT End"),
                ]);
                for (budget, budget_recurrence) in r {
                    let dates: (String, String) = budget_recurrence
                        .map(|mut v| {
                            let rrule_dt_start =
                                v.dt_start.inner().and_local_timezone(Tz::UTC).unwrap();
                            let validated = v.recurrence_rule.validate(rrule_dt_start).unwrap();
                            let until =
                                validated.get_until().expect("Validate logic should set UNTIL");
                            return (
                                v.dt_start.inner().format("%Y-%m-%d").to_string(),
                                until.format("%Y-%m-%d").to_string(),
                            );
                        })
                        .unwrap_or_else(|| ("".to_string(), "".to_string()));
                    table.push_row(vec![
                        budget.id.to_string().as_str(),
                        &budget.name,
                        &budget.description.unwrap_or_else(|| "".to_string()),
                        &format_money_usd(budget.anticipated_amount),
                        &dates.0,
                        &dates.1,
                    ]);
                }
                table.display();
            }
            Err(e) => {
                alert!("Failed to list budgets");
                log::error!("List Budgets: {}", e)
            }
        },
    }
}
