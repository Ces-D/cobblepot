use crate::chart_of_accounts::models::{Account, AccountCategory, AccountEntry};
use crate::chart_of_accounts::store;
use chrono;

mod chart_of_accounts;
mod cobblepot_core;

// struct SessionAccount {
//     branch: Account,
// }
//
// fn run() {
//     let path = cobblepot_core::chart_of_accounts_path();
//
//     let account = Account {
//         company: Some(String::from("Capital One")),
//         name: String::from("Checking"),
//         description: String::from("My primary bank account"),
//         category: chart_of_accounts::AccountCategory::Asset,
//     };
//     let account_entry = AccountEntry { opened: chrono::Utc::now(), closed: None, account };
//     let another_account = Account {
//         company: Some(String::from("Vanguard")),
//         name: String::from("Roth IRA"),
//         description: String::from("My retirement investment account"),
//         category: chart_of_accounts::AccountCategory::Asset,
//     };
//     let another_account_entry =
//         AccountEntry { opened: chrono::Utc::now(), closed: None, account: another_account };
//
//     let chart_of_accounts = ChartOfAccounts { entries: vec![account_entry, another_account_entry] };
//     let json_chart_of_accounts = serde_json::to_string_pretty(&chart_of_accounts).expect("Failure");
//
//     let chart_of_accounts_file =
//         OpenOptions::new().append(true).open(std::path::Path::new(path)).expect("WTF");
//     let mut writer = BufWriter::new(chart_of_accounts_file);
//     match writer.write_all(json_chart_of_accounts.as_bytes()) {
//         Ok(_) => println!("Success"),
//         Err(_) => println!("Failure"),
//     }
// }

fn run() {
    let account = Account {
        company: Some(String::from("Capital One")),
        name: String::from("Checking"),
        description: String::from("My primary bank account"),
        category: AccountCategory::Asset,
    };
    let account_entry = AccountEntry { opened: chrono::Utc::now(), closed: None, account };
    let another_account = Account {
        company: Some(String::from("Vanguard")),
        name: String::from("Roth IRA"),
        description: String::from("My retirement investment account"),
        category: AccountCategory::Asset,
    };
    let another_account_entry =
        AccountEntry { opened: chrono::Utc::now(), closed: None, account: another_account };

    store::add_account(account_entry);
    store::add_account(another_account_entry);
}

fn main() {
    run();
    // let mut input = String::new();
    // let acceptable_account = SessionAccount {
    //     branch: Account::create(
    //         String::from("Capital One Account"),
    //         String::from("A bank account"),
    //         AccountCategory::Asset,
    //     ),
    // };
    //
    // println!("Temp: Type in your branch");
    //
    // let account = match io::stdin().read_line(&mut input) {
    //     Ok(_) => input.trim().to_string(),
    //     Err(_) => input.trim().to_string(),
    // };
    //
    // if acceptable_account.branch.name == account {
    //     println!(
    //         "You have selected {0} - {1}",
    //         acceptable_account.branch.name, acceptable_account.branch.description
    //     );
    //     let completed =
    //         files::write_to_chart_of_accounts(acceptable_account.branch.to_csv_string());
    //     if completed {
    //         println!("Success")
    //     } else {
    //         println!("Failure")
    //     }
    // } else {
    //     println!("This account is not acceptable {0}", input)
    // }
}

// TODO: test the add_account function in chart_of_accounts::store
