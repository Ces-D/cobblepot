use clap::{Parser, Subcommand};
use diesel::SqliteConnection;

#[derive(Debug, Subcommand)]
enum ReportCommand {
    BalanceSheet {
        start: Option<chrono::NaiveDateTime>,
        end: Option<chrono::NaiveDateTime>,
    },
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
    todo!()
}
