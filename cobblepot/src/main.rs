use cobblepot_journal::journal;
use std::io;

struct Account {
    branch: String,
}

fn main() {
    let mut input = String::new();
    println!("Temp: Type in your branch");

    let account = match io::stdin().read_line(&mut input) {
        Ok(_) => Account { branch: input.trim().to_string() },
        Err(_) => Account { branch: input.trim().to_string() },
    };

    if account.branch.is_empty() {
        println!("You must initialize your sessions branch")
    } else {
        println!("You have selected {0}", account.branch)
    }
}

// TODO: integrate create cobblepot_journal into this and test. Might need to rename local struct
// Account
