use std::env;

#[cfg(feature = "dotenv")]
use dotenvy;
use tracing::{debug, info, warn};

pub struct Env;

impl Env {
    #[cfg(feature = "dotenv")]
    pub fn load_dotenv() -> Result<(), u8> {
        match dotenvy::dotenv() {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!(
                    "An error occurred wile trying to load the .env file: {}",
                    &err
                );
                Err(0x1)
            }
        }
    }

    pub fn validate() -> Result<(), u8> {
        match env::var("LOG_STDOUT_LEVEL") {
            Ok(value) => match &(*value.to_uppercase()) {
                "OFF" | "ERROR" | "WARN" | "INFO" | "DEBUG" | "TRACE" => {
                    debug!(
                        "LOG_STDOUT_LEVEL environment variable was found: {}",
                        &value
                    );
                    info!("Used stdout log level: {}", &value.to_uppercase());
                }
                _ => {
                    warn!("LOG_STDOUT_LEVEL environment variable was found but did not match any known levels: {}", &value);
                    info!("Used stdout log level: INFO");
                }
            },
            Err(_) => {
                debug!("LOG_STDOUT_LEVEL environment variable was not found");
                info!("Used stdout log level: INFO");
            }
        };

        match env::var("LOG_FILES_LEVEL") {
            Ok(value) => match &(*value.to_uppercase()) {
                "OFF" | "ERROR" | "WARN" | "INFO" | "DEBUG" | "TRACE" => {
                    debug!("LOG_FILES_LEVEL environment variable was found: {}", &value);
                    info!("Used files log level: {}", &value.to_uppercase());
                }
                _ => {
                    warn!("LOG_FILES_LEVEL environment variable was found but did not match any known levels: {}", &value);
                    info!("Used files log level: INFO");
                }
            },
            Err(_) => {
                debug!("LOG_FILES_LEVEL environment variable was not found");
                info!("Used files log level: INFO");
            }
        };

        Ok(())
    }
}
