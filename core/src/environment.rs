use log::info;
use std::{env::VarError, path::Path};

const COBBLEPOT_DB_NAME_VAR: &str = "COBBLEPOT_DB_NAME";
const COBBLEPOT_TIINGO_API_KEY_VAR: &str = "COBBLEPOT_TIINGO_API_KEY";

const COBBLEPOT_LOCAL_DATA_PATH: &str = ".local/share/cobblepot";

/// The production database URL. Can adjust db name with the optional `COBBLEPOT_DB_NAME` env var
pub fn database_url() -> Result<String, VarError> {
    let home_dir = std::env::var("HOME")?;
    let cobblepot_db_name =
        std::env::var(COBBLEPOT_DB_NAME_VAR).unwrap_or("cobblepot.db".to_string());

    let storage_dir = Path::new(&home_dir).join(COBBLEPOT_LOCAL_DATA_PATH);
    if !storage_dir.exists() {
        info!("Creating application storage directory at {}", storage_dir.display());
        std::fs::create_dir_all(&storage_dir).expect("Failed to create data local directory");
    }
    let db_path = storage_dir.join(cobblepot_db_name);
    Ok(format!("{}", db_path.display()))
}

/// The Tiingo fincial markets api key. Can set using the `COBBLEPOT_TIINGO_API_KEY` env var
pub fn financial_api_key() -> Result<String, VarError> {
    std::env::var(COBBLEPOT_TIINGO_API_KEY_VAR)
}
