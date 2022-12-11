use clap::Command;
use cobblepot_core::chart_of_accounts::ChartOfAccounts;
use std::env;
use std::io::{Result, Write};
use std::time::Duration;

mod chart_of_accounts_command;
mod cobblepot_command;
mod shared;

const WORKING_ACCOUNT_ENV_KEY: &'static str = "COBBLEPOT_ACCOUNT";

fn set_working_account(account_name: &String) {
    env::set_var(WORKING_ACCOUNT_ENV_KEY, account_name)
}

fn is_valid_working_account() -> bool {
    let chart_of_accounts = ChartOfAccounts::read_from_store_or_create();

    let working_account =
        env::var(WORKING_ACCOUNT_ENV_KEY).expect("You did not set the working account env");
    chart_of_accounts.list_entries().contains(&working_account)
}

// fn run() {
//     let account = Account {
//         company: Some(String::from("Capital One")),
//         name: String::from("Checking"),
//         description: String::from("My primary bank account"),
//         category: AccountCategory::Asset,
//     };
//     let another_account = Account {
//         company: Some(String::from("Vanguard")),
//         name: String::from("Roth IRA"),
//         description: String::from("My retirement investment account"),
//         category: AccountCategory::Asset,
//     };
//
//     let mut chart_of_accounts = ChartOfAccounts::new();
//
//     chart_of_accounts.open_account(account).expect("First Account opening failed");
//     chart_of_accounts.open_account(another_account).expect("Second Account opening failed");
//
//     match chart_of_accounts.save() {
//         Ok(_) => println!("Save successful"),
//         Err(_) => println!("Unable to save"),
//     }
// }

fn run() -> Result<()> {
    let cli = commands_args::cobblepot_command::create_cobblepot_command_app().get_matches();

    match cli.subcommand() {
        Some(("chart_of_accounts", chart_of_accounts_matches)) => {
            commands_args::chart_of_accounts_command::handle(chart_of_accounts_matches)
        },
        _ => Ok(()),
    }
}

fn main() {
    run().unwrap();
}
