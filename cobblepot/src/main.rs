use clap::{Parser, Subcommand};
use dotenvy::dotenv;

mod client;
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
        about = "Calculate current BalanceSheet.",
        long_about = "Calculates the current balance of account types based on the currently opened accounts."
    )]
    BalanceSheet,
    #[command(
        about = "Calculate Deltas of account metrics",
        long_about = "Calculates the deltas of account metrics. Metrics could include growth over time or difference across accounts"
    )]
    Delta,
}

pub fn main() {
    dotenv().ok();
    let connection = client::establish_connection();

    // TODO: test the account commands
    match Cli::parse().command {
        Commands::NewAccount(new_account) => {
            match client::account::query::insert_new_account(new_account, connection) {
                Ok(account) => {
                    let table = tabled::Table::new(vec![account]);
                    println!("{}", table);
                },
                Err(err) => println!("Error: {}", err),
            }
        },
        Commands::EditAccount(edit_account) => {
            match client::account::query::update_account(edit_account, connection) {
                Ok(account) => {
                    let table = tabled::Table::new(vec![account]);
                    println!("{}", table);
                },
                Err(err) => println!("Error: {}", err),
            }
        },
        Commands::ListAccounts => match client::account::query::list_accounts(connection) {
            Ok(accounts) => {
                let table = tabled::Table::new(accounts);
                println!("{}", table);
            },
            Err(err) => println!("Error: {}", err),
        },
        Commands::NewBalance(new_balance) => {
            match client::balance::query::insert_new_balance(new_balance, connection) {
                Ok(balance) => {
                    let table = tabled::Table::new(vec![balance]);
                    println!("{}", table);
                },
                Err(err) => println!("Error: {}", err),
            }
        },
        Commands::EditBalance(edit_balance) => {
            match client::balance::query::update_balance(edit_balance, connection) {
                Ok(balance) => {
                    let table = tabled::Table::new(vec![balance]);
                    println!("{}", table);
                },
                Err(err) => println!("Error: {}", err),
            }
        },
        Commands::ListBalances => match client::balance::query::list_balances(connection) {
            Ok(balances) => {
                let table = tabled::Table::new(balances);
                println!("{}", table);
            },
            Err(err) => println!("Error: {}", err),
        },
        Commands::BalanceSheet => todo!(),
        Commands::Delta => todo!(),
    }
}
