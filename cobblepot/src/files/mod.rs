use std::fs;

const COBBLEPOT_CORE: &str = "~/Documents/Cobblepot/";
const CHART_OF_ACCOUNTS: &str = "~/Documents/Cobblepot/chart_of_accounts.txt";

fn ensure_core_dir_exists() -> bool {
    let result = match fs::create_dir_all(COBBLEPOT_CORE) {
        Ok(_) => true,
        Err(_) => false,
    };
    result
}

fn ensure_chart_of_accounts_exists() -> bool {
    let result = match fs::File::create(CHART_OF_ACCOUNTS) {
        Ok(_) => true,
        Err(_) => false,
    };
    result
}


