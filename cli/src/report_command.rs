use crate::{alert, success};
use clap::{Parser, Subcommand};
use diesel::SqliteConnection;

mod balance_sheet;
mod dto;

#[derive(Debug, Subcommand)]
enum ReportCommand {
    BalanceSheet,
    AccountDive {
        #[clap(help = "Id of the account")]
        account_id: i32,
    },
}

#[derive(Debug, Parser)]
pub struct ReportArgs {
    #[clap(subcommand)]
    pub command: ReportCommand,
}

pub fn handle_report_command(args: ReportArgs, conn: SqliteConnection) {
    match args.command {
        ReportCommand::BalanceSheet => match dto::get_open_accounts_with_latest_balance(conn) {
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
        },
        ReportCommand::AccountDive {
            account_id,
        } => todo!(),
    }
}
