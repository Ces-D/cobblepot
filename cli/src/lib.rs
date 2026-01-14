mod create_command;
mod list_command;
mod logger;
mod report_command;
mod shared;
mod update_command;

use clap::{Parser, Subcommand};
use diesel::Connection;

#[derive(Debug, Subcommand)]
enum Command {
    List(list_command::ListArgs),
    Create(create_command::CreateArgs),
    Report(report_command::ReportArgs),
    Update(update_command::UpdateArgs),
}

#[derive(Debug, clap::Parser)]
#[clap(author, version, bin_name = "cobblepot", subcommand_required = true)]
struct App {
    #[clap(subcommand)]
    pub command: Command,
}

pub fn main() {
    let is_production = init_logging();
    cobblepot_core::Config::init(is_production);
    let conn =
        diesel::SqliteConnection::establish(&cobblepot_core::Config::global().database_url())
            .unwrap();
    let app = App::parse();

    match app.command {
        Command::List(list_args) => list_command::handle_list_command(list_args, conn),
        Command::Create(create_args) => create_command::handle_create_command(create_args, conn),
        Command::Report(report_args) => report_command::handle_report_command(report_args, conn),
        Command::Update(update_args) => update_command::handle_update_command(update_args, conn),
    }
}

/// Returns a bool signifying that RUST_LOG=="info".
/// This will proxy as in production
fn init_logging() -> bool {
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
    level == "info"
}
