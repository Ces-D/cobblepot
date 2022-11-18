use crate::chart_of_accounts::{Account, AccountCategory, ChartOfAccounts};

mod chart_of_accounts;
mod cobblepot_core;

fn run() {
    let account = Account {
        company: Some(String::from("Capital One")),
        name: String::from("Checking"),
        description: String::from("My primary bank account"),
        category: AccountCategory::Asset,
    };
    let another_account = Account {
        company: Some(String::from("Vanguard")),
        name: String::from("Roth IRA"),
        description: String::from("My retirement investment account"),
        category: AccountCategory::Asset,
    };

    let mut chart_of_accounts = ChartOfAccounts::new();

    chart_of_accounts.open_account(account).expect("First Account opening failed");
    chart_of_accounts.open_account(another_account).expect("Second Account opening failed");

    for entry in chart_of_accounts.list_entries() {
        println!("{} is an account", entry)
    }

    match chart_of_accounts.close_account("Checking".to_string()) {
        Ok(_) => println!("Closed Checking"),
        Err(_) => println!("Failed to Closed Checking"),
    }

    match chart_of_accounts.save() {
        Ok(_) => println!("Save successful"),
        Err(_) => println!("Unable to save"),
    }
}

fn main() {
    run();
}
