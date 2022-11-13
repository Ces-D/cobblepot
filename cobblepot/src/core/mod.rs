use std::io::Result;
use std::path::Path;

static COBBLEPOT_STORE: &'static str = "./Cobblepot_Store/";
static CHART_OF_ACCOUNTS: &'static str = "./Cobblepot_Store/chart_of_accounts.json";

fn ensure_store_dir_exists() -> Result<()> {
    let cobblepot_store_path = Path::new(COBBLEPOT_STORE);
    if !cobblepot_store_path.exists() {
        std::fs::create_dir_all(cobblepot_store_path)
    } else {
        Ok(())
    }
}

fn ensure_chart_of_accounts_file_exists() -> Result<()> {
    let chart_accounts_path = Path::new(CHART_OF_ACCOUNTS);
    if !chart_accounts_path.exists() {
        std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(chart_accounts_path)
            .expect("Unable to Create Chart of Accounts File");
        Ok(())
    } else {
        Ok(())
    }
}

pub fn chart_of_accounts_path() -> &'static str {
    ensure_store_dir_exists();
    ensure_chart_of_accounts_file_exists();
    CHART_OF_ACCOUNTS
}
