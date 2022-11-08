use std::fs::{File, OpenOptions};
use std::io::{Result, Write};
use std::path::Path;

const COBBLEPOT_CORE: &str = "./Cobblepot_Storage/";
const CHART_OF_ACCOUNTS: &str = "./Cobblepot_Storage/chart_of_accounts.txt";

fn ensure_core_dir_exists() -> Result<()> {
    std::fs::create_dir_all(Path::new(COBBLEPOT_CORE))
}

fn ensure_chart_of_accounts_exists() -> Result<File> {
    OpenOptions::new().write(true).create_new(true).open(Path::new(CHART_OF_ACCOUNTS))
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
