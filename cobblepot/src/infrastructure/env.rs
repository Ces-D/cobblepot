use crate::shared::{CobblepotError, CobblepotResult};
use log::info;
use std::{env::VarError, path::Path};

pub fn is_production() -> CobblepotResult<bool> {
    match std::env::var("IS_PRODUCTION") {
        Ok(env) => {
            if env == "true" {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(error) => match error {
            VarError::NotPresent => Ok(false),
            VarError::NotUnicode(invalid) => {
                Err(CobblepotError::from(VarError::NotUnicode(invalid)))
            }
        },
    }
}

pub fn database_url() -> CobblepotResult<String> {
    let is_prod = is_production()?;

    match is_prod {
        true => {
            let home_dir = std::env::var("HOME")?;
            let storage_dir = Path::new(&home_dir).join(".local/share/cobblepot");
            if !storage_dir.exists() {
                info!("Creating application storage directory at {}", storage_dir.display());
                std::fs::create_dir_all(&storage_dir)
                    .expect("Failed to create data local directory");
            }
            let db_path = storage_dir.join("cobblepot.db");
            Ok(format!("{}", db_path.display()))
        }
        false => Ok(":memory".to_string()),
    }
}
