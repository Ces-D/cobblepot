use crate::chart_of_accounts::models::{AccountEntry, ChartOfAccounts};
use crate::cobblepot_core;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Result, Write};

fn open_chart_of_accounts_writer() -> BufWriter<std::fs::File> {
    let path = cobblepot_core::chart_of_accounts_path();
    let f = OpenOptions::new().write(true).open(path).expect("WTF");
    BufWriter::new(f)
}

fn open_chart_of_accounts_reader() -> BufReader<std::fs::File> {
    let path = cobblepot_core::chart_of_accounts_path();
    let f = OpenOptions::new().read(true).open(path).expect("WTF");
    BufReader::new(f)
}

fn read_store() -> ChartOfAccounts {
    let reader = open_chart_of_accounts_reader();
    let chart_of_accounts = match serde_json::from_reader(reader) {
        Ok(data) => data,
        Err(_) => ChartOfAccounts { entries: vec![] },
    };
    chart_of_accounts
}

pub fn add_account(entry: AccountEntry) -> Result<()> {
    let mut chart_of_accounts = read_store();
    chart_of_accounts.entries.push(entry);

    let json_chart_of_accounts = serde_json::to_string_pretty(&chart_of_accounts).expect("Failure");

    // let path = cobblepot_core::chart_of_accounts_path();
    // let f = OpenOptions::new().write(true).open(path).expect("WTF");
    // let mut writer = BufWriter::new(f);
    let mut writer = open_chart_of_accounts_writer();
    match writer.write(json_chart_of_accounts.as_bytes()) {
        Ok(_) => writer.flush(),
        Err(_) => writer.flush(),
    }
}
