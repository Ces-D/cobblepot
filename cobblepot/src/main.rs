use cobblepot_journal::account::{Account, AccountCategory};
use std::io;

struct SessionAccount {
    branch: Account,
}

fn main() {
    let mut input = String::new();
    let acceptable_account = SessionAccount {
        branch: Account::create(
            String::from("Capital One Account"),
            String::from("A bank account"),
            AccountCategory::Asset,
        ),
    };

    println!("Temp: Type in your branch");

    let account = match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(_) => input.trim().to_string(),
    };

    if acceptable_account.branch.name == account {
        println!(
            "You have selected {0} - {1}",
            acceptable_account.branch.name, acceptable_account.branch.description
        )
    } else {
        println!("This account is not acceptable {0}", input)
    }
}
