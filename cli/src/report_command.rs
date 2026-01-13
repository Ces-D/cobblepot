use crate::{alert, success};
use clap::{Parser, Subcommand};
use diesel::SqliteConnection;

mod balance_sheet;
mod budget;
mod dto;

#[derive(Debug, Subcommand)]
pub enum ReportCommand {
    #[clap(about = "View your accounts current balance sheet")]
    BalanceSheet,
    AccountDive {
        #[clap(help = "Id of the account")]
        account_id: i32,
    },
    #[clap(about = "View a budget report")]
    Budget {
        #[clap(help = "Id of the budget")]
        budget_id: i32,
    },
}

#[derive(Debug, Parser)]
pub struct ReportArgs {
    #[clap(subcommand)]
    pub command: ReportCommand,
}

pub fn handle_report_command(args: ReportArgs, mut conn: SqliteConnection) {
    match args.command {
        ReportCommand::BalanceSheet => {
            match dto::get_open_accounts_with_latest_balance(&mut conn) {
                Ok(r) => {
                    success!("Generating balance sheet from {} entries", r.len());
                    let mut balance_sheet = balance_sheet::BalanceSheet::default();
                    for row in r {
                        balance_sheet.push_balance(row);
                    }
                    balance_sheet.display();
                }
                Err(e) => {
                    alert!("Failed to make balance sheet");
                    log::error!("Balance Sheet Error: {}", e)
                }
            }
        }
        ReportCommand::AccountDive {
            account_id: _,
        } => todo!(),
        ReportCommand::Budget {
            budget_id,
        } => {
            let accounts_with_latest_balance =
                dto::get_open_accounts_with_latest_balance(&mut conn)
                    .inspect_err(|e| log::error!("Error getting budget accounts: {}", e))
                    .expect("This shouldn't fail");
            let budget_data = dto::get_budget_data(conn, budget_id)
                .inspect_err(|e| log::error!("Error getting budget data: {}", e))
                .expect("This also shouldn't fail");
            let mut report = budget::BudgetReport::from_data(budget_data);
            report.set_account_latest_balances(accounts_with_latest_balance);
            report.display();
        }
    }
}
