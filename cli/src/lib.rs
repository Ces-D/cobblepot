mod create_command;
mod list_command;
mod logger;
mod report_command;
mod shared;

use clap::{Parser, Subcommand};
use diesel::Connection;

#[derive(Debug, Subcommand)]
enum Command {
    List(list_command::ListArgs),
    Create(create_command::CreateArgs),
    Report(report_command::ReportArgs),
}

#[derive(Debug, clap::Parser)]
#[clap(author, version, bin_name = "cobblepot", subcommand_required = true)]
struct App {
    #[clap(subcommand)]
    pub command: Command,
}

pub fn main(database_url: String) {
    init_logging();
    let conn = diesel::SqliteConnection::establish(&database_url).unwrap();
    let app = App::parse();

    match app.command {
        Command::List(list_args) => list_command::handle_list_command(list_args, conn),
        Command::Create(create_args) => create_command::handle_create_command(create_args, conn),
        Command::Report(report_args) => report_command::handle_report_command(report_args, conn),
    }
}

fn init_logging() {
    const MEMBERS: [&str; 4] = ["cli", "core", "data_store", "financial_markets"];

    // Get global log level from env or use default
    let level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

    // Build filter string applying the same level to all modules
    let filters = MEMBERS.iter().map(|m| format!("{m}={level}")).collect::<Vec<_>>().join(",");

    let env = env_logger::Env::default()
        .filter_or("RUST_LOG", &filters)
        .write_style_or("RUST_LOG_STYLE", "always");

    env_logger::Builder::from_env(env).init();

    log::warn!("Logging initialized with level: {level}");
}
