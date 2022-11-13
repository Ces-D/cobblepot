use std::fs::OpenOptions;
use std::io::{Result, Write};
use std::path::Path;

const COBBLEPOT_CORE: &str = "./Cobblepot_Storage/";
const CHART_OF_ACCOUNTS: &str = "./Cobblepot_Storage/chart_of_accounts.csv";

fn ensure_core_dir_exists() -> Result<()> {
    std::fs::create_dir_all(Path::new(COBBLEPOT_CORE))
}

fn ensure_chart_of_accounts_exists() -> Result<()> {
    let chart_of_account_path = Path::new(CHART_OF_ACCOUNTS);
    if !chart_of_account_path.exists() {
        OpenOptions::new()
            .append(true)
            .create_new(true)
            .open(chart_of_account_path)
            .expect("Unable to Create Chart of Accounts File");
        Ok(())
    } else {
        Ok(())
    }
}

pub fn write_to_chart_of_accounts(line: String) -> bool {
    match ensure_core_dir_exists() {
        Ok(_) => match ensure_chart_of_accounts_exists() {
            Ok(_) => {
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(Path::new(CHART_OF_ACCOUNTS))
                    .expect("cannot open file");
                file.write_all(line.as_bytes()).expect("Could not write to file");
                true
            },
            Err(_) => false,
        },
        Err(_) => false,
    }
}
