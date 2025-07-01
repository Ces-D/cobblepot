use crate::shared::CobblepotResult;
use log::info;
use std::path::Path;

/// The production database URL.
pub fn database_url() -> CobblepotResult<String> {
    let home_dir = std::env::var("HOME")?;
    let cobblepot_db_name =
        std::env::var("COBBLEPOT_DB_NAME").unwrap_or("cobblepot.db".to_string());

    let storage_dir = Path::new(&home_dir).join(".local/share/cobblepot");
    if !storage_dir.exists() {
        info!("Creating application storage directory at {}", storage_dir.display());
        std::fs::create_dir_all(&storage_dir).expect("Failed to create data local directory");
    }
    let db_path = storage_dir.join(cobblepot_db_name);
    Ok(format!("{}", db_path.display()))
}
