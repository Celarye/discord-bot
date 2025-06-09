use std::{
    collections::{BTreeMap, HashMap},
    fs::read,
    path::PathBuf,
};

use serde::Deserialize;
use serde_json::Value;
use tracing::error;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub name: Option<String>,
    pub version: Option<String>,
    #[serde(default = "Config::default_cache")]
    pub cache: bool,
    #[serde(default = "Config::default_directory")]
    pub directory: PathBuf,
    pub plugins: BTreeMap<String, ConfigPluginValues>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigPluginValues {
    pub version: String,
    pub environment: Option<HashMap<String, String>>,
    pub settings: Option<Value>,
}

impl Config {
    pub fn new(file_path: &PathBuf) -> Result<Self, ()> {
        let file_bytes = match read(file_path) {
            Ok(file_bytes) => file_bytes,
            Err(err) => {
                error!("Failed to read the config file: {}", &err);
                return Err(());
            }
        };

        match serde_yaml_ng::from_slice(&file_bytes) {
            Ok(config) => Ok(config),
            Err(err) => {
                error!(
                    "An error occurred while deserializing the config file YAML to a struct: {}",
                    &err
                );
                Err(())
            }
        }
    }

    fn default_cache() -> bool {
        true
    }

    fn default_directory() -> PathBuf {
        PathBuf::from("./plugins/")
    }
}
