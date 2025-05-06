use clap::{Parser, Subcommand};
use tabled::settings::style::BorderColor;
use tabled::settings::{Color, Style};

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
    NewAccount(client::model::NewAccount),
    EditAccount(client::model::EditAccount),
    #[command(about = "List Accounts")]
    ListAccounts,
    NewBalance(client::model::NewBalance),
    EditBalance(client::model::EditBalance),
    #[command(about = "List balance entries of an account")]
    ListBalances {
        #[arg(long, help = "Account ID to filter by")]
        account_id: Option<i32>,
    },
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
                    let table = create_table(vec![account]);
                    print_table(table);
                }
                Err(err) => println!("Error: {}", err),
            }
        }
        Commands::EditAccount(edit_account) => {
            match client::account::query::update_account(edit_account, connection) {
                Ok(account) => {
                    let table = create_table(vec![account]);
                    print_table(table);
                }
                Err(err) => println!("Error: {}", err),
            }
        }
        Commands::ListAccounts => match client::account::query::list_accounts(connection) {
            Ok(accounts) => {
                let table = create_table(accounts);
                print_table(table);
            }
            Err(err) => println!("Error: {}", err),
        },
        Commands::NewBalance(new_balance) => {
            match client::balance::query::insert_new_balance(new_balance, connection) {
                Ok(balance) => {
                    let table = create_table(vec![balance]);
                    print_table(table);
                }
                Err(err) => println!("Error: {}", err),
            }
        }
        Commands::EditBalance(edit_balance) => {
            match client::balance::query::update_balance(edit_balance, connection) {
                Ok(balance) => {
                    let table = create_table(vec![balance]);
                    print_table(table);
                }
                Err(err) => println!("Error: {}", err),
            }
        }
        Commands::ListBalances { account_id } => {
            match client::balance::query::list_balances(connection, account_id) {
                Ok(balances) => {
                    let table = create_table(balances);
                    print_table(table);
                }
                Err(err) => println!("Error: {}", err),
            }
        }
        Commands::BalanceSheet(balance_sheet_command) => {
            match client::balance_sheet::query::get_balances(balance_sheet_command, connection) {
                Ok(balance_sheet) => {
                    println!("Balance Sheet");
                    println!("From: {}", balance_sheet.from);
                    println!("To: {}", balance_sheet.to);

                    println!("Current Assets");
                    let current_assets_table = create_table(balance_sheet.current_assets);
                    print_table(current_assets_table);
                    println!("Current Liabilities");
                    let current_liabilities_table = create_table(balance_sheet.current_liabilities);
                    print_table(current_liabilities_table);
                    println!("Non-Current Assets");
                    let non_current_assets_table = create_table(balance_sheet.non_current_assets);
                    print_table(non_current_assets_table);
                    println!("Non-Current Liabilities");
                    let non_current_liabilities_table =
                        create_table(balance_sheet.non_current_liabilities);
                    print_table(non_current_liabilities_table);

                    println!("Net Assets: {}", balance_sheet.net_assets);
                    println!("Net Liabilities: {}", balance_sheet.net_liabilities);
                }
                Err(err) => println!("Error: {}", err),
            }
        }
        Commands::DeepDive { account_id } => {
            match client::deep_dive::query::get_analytics(account_id, connection) {
                Ok(analysis) => {
                    println!("");
                    let writer = std::io::stdout();
                    serde_json::to_writer_pretty(writer, &analysis).expect("Failed to write JSON");
                }
                Err(err) => println!("Error: {}", err),
            }
        }
    }
}

fn create_table<I, T>(iter: I) -> tabled::Table
where
    I: IntoIterator<Item = T>,
    T: tabled::Tabled,
{
    let mut t = tabled::Table::new(iter);
    t.with(Style::rounded());
    t.with(BorderColor::filled(Color::BG_WHITE));
    t
}

fn print_table(t: tabled::Table) {
    println!("");
    println!("{}", t);
    println!("")
}
