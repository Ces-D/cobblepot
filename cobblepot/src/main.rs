use clap::{Parser, Subcommand};

mod client;
mod config;
mod schema;

#[derive(Parser)]
#[command(version, about="A personal finance journal", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    NewAccount(client::account::NewAccount),
    EditAccount(client::account::EditAccount),
    #[command(about = "List Accounts")]
    ListAccounts,
    NewBalance(client::balance::NewBalance),
    EditBalance(client::balance::EditBalance),
    #[command(about = "List balance entries of an account")]
    ListBalances,
    #[command(
        about = "Calculate a BalanceSheet",
        long_about = "Summarize the balance of accounts based on which are currently opened"
    )]
    BalanceSheet(client::balance_sheet::BalanceSheetCommand),
    #[command(
        about = "Dive deep into account analytics",
        long_about = "Creative and insightful analytics on an account and its balance history"
    )]
    DeepDive {
        account_id: i32,
    },
}

pub fn main() {
    let config = config::Config::setup();
    let connection = config.establish_connection();

    match Cli::parse().command {
        Commands::NewAccount(new_account) => {
            match client::account::query::insert_new_account(new_account, connection) {
                Ok(account) => {
                    let table = tabled::Table::new(vec![account]);
                    println!("{}", table);
                }
                Err(err) => println!("Error: {}", err),
            }
        }
        Commands::EditAccount(edit_account) => {
            match client::account::query::update_account(edit_account, connection) {
                Ok(account) => {
                    let table = tabled::Table::new(vec![account]);
                    println!("{}", table);
                }
                Err(err) => println!("Error: {}", err),
            }
        }
        Commands::ListAccounts => match client::account::query::list_accounts(connection) {
            Ok(accounts) => {
                let table = tabled::Table::new(accounts);
                println!("{}", table);
            }
            Err(err) => println!("Error: {}", err),
        },
        Commands::NewBalance(new_balance) => {
            match client::balance::query::insert_new_balance(new_balance, connection) {
                Ok(balance) => {
                    let table = tabled::Table::new(vec![balance]);
                    println!("{}", table);
                }
                Err(err) => println!("Error: {}", err),
            }
        }
        Commands::EditBalance(edit_balance) => {
            match client::balance::query::update_balance(edit_balance, connection) {
                Ok(balance) => {
                    let table = tabled::Table::new(vec![balance]);
                    println!("{}", table);
                }
                Err(err) => println!("Error: {}", err),
            }
        }
        Commands::ListBalances => match client::balance::query::list_balances(connection) {
            Ok(balances) => {
                let table = tabled::Table::new(balances);
                println!("{}", table);
            }
            Err(err) => println!("Error: {}", err),
        },
        Commands::BalanceSheet(balance_sheet_command) => {
            match client::balance_sheet::query::get_balances(balance_sheet_command, connection) {
                Ok(balance_sheet) => {
                    println!("Balance Sheet");
                    println!("From: {}", balance_sheet.from);
                    println!("To: {}", balance_sheet.to);

                    println!("Current Assets");
                    let current_assets_table = tabled::Table::new(balance_sheet.current_assets);
                    println!("{}", current_assets_table);
                    println!("Current Liabilities");
                    let current_liabilities_table =
                        tabled::Table::new(balance_sheet.current_liabilities);
                    println!("{}", current_liabilities_table);
                    println!("Non-Current Assets");
                    let non_current_assets_table =
                        tabled::Table::new(balance_sheet.non_current_assets);
                    println!("{}", non_current_assets_table);
                    println!("Non-Current Liabilities");
                    let non_current_liabilities_table =
                        tabled::Table::new(balance_sheet.non_current_liabilities);
                    println!("{}", non_current_liabilities_table);
                }
                Err(err) => println!("Error: {}", err),
            }
        }
        Commands::DeepDive { account_id } => {
            match client::deep_dive::query::get_analytics(account_id, connection) {
                Ok(analysis) => {
                    let writer = std::io::stdout();
                    serde_json::to_writer(writer, &analysis).expect("Failed to write JSON");
                }
                Err(err) => println!("Error: {}", err),
            }
        }
    }
}
