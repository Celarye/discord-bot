use axum::body::Bytes;
use serde_json::Value;
use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::Path,
    time::Duration,
};
use tracing::error;

use reqwest::{
    Client, StatusCode,
    header::{HeaderMap, HeaderValue},
};

use crate::config::ConfigPluginValues;

pub struct Requests {
    client: Client,
}

#[derive(Clone, Debug)]
pub struct AvailablePlugin {
    pub name: String,
    pub version: String,
    pub environment: Option<HashMap<String, String>>,
    pub settings: Option<Value>,
}

impl Requests {
    pub fn new() -> Result<Requests, ()> {
        match Client::builder()
            .user_agent("celarye/discord-bot")
            .timeout(Duration::new(15, 0))
            .build()
        {
            Ok(client) => Ok(Requests { client }),
            Err(err) => {
                error!(
                    "Something went wrong while creating the request client: {}",
                    &err
                );
                Err(())
            }
        }
    }

    pub async fn get_file_from_registry(
        &self,
        registry: &String,
        path: &String,
    ) -> Result<Bytes, ()> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_str("application/vnd.github.raw+json").unwrap(),
        );
        headers.insert(
            "X-GitHub-Api-Version",
            HeaderValue::from_str("2022-11-28").unwrap(),
        );

        match self
            .client
            .get(format!(
                "https://api.github.com/repos/{}/contents/{}",
                &registry, &path
            ))
            .headers(headers)
            .send()
            .await
        {
            Ok(raw_response) => {
                match raw_response.status() {
                    StatusCode::OK => (),
                    _ => {
                        error!(
                            "The response was undesired, status code: {}",
                            &raw_response.status(),
                        );
                        return Err(());
                    }
                }

                match raw_response.bytes().await {
                    Ok(response) => Ok(response),
                    Err(err) => {
                        error!(
                            "Something went wrong while getting the raw bytes from the response, error: {}",
                            &err
                        );
                        Err(())
                    }
                }
            }
            Err(err) => {
                error!(
                    "Something went wrong while making the request, error: {}",
                    &err
                );
                Err(())
            }
        }
    }

    pub async fn fetch_plugins(
        &self,
        plugins: &BTreeMap<String, ConfigPluginValues>,
        directory: &Path,
        cache: bool,
    ) -> Result<Vec<AvailablePlugin>, ()> {
        let mut available_plugins = vec![];

        for (plugin, plugin_settings) in plugins {
            let (registry, name) = match plugin.rsplit_once("/") {
                None => (&String::from("celarye/discord-bot-plugins"), plugin),
                Some((registry, plugin)) => (&registry.to_string(), &plugin.to_string()),
            };

            let plugin_dir = directory.join(&name).join(&plugin_settings.version);

            if cache && fs::exists(plugin_dir.join("plugin.wasm")).unwrap_or_else(|_| false) {
                available_plugins.push(AvailablePlugin {
                    name: name.clone(),
                    version: plugin_settings.version.clone(),
                    environment: plugin_settings.environment.clone(),
                    settings: plugin_settings.settings.clone(),
                });
                continue;
            }

            let Ok(plugin_metadata) = self
                .get_file_from_registry(
                    &registry,
                    &format!("{}/{}/metadata.json", &name, &plugin_settings.version),
                )
                .await
            else {
                continue;
            };

            if let Err(err) =
                fs::create_dir_all(directory.join(&name).join(&plugin_settings.version))
            {
                error!(
                    "Something went wrong while creating the plugin directory, error: {}",
                    err
                );
                return Err(());
            }

            if let Err(err) = fs::write(
                directory
                    .join(&name)
                    .join(&plugin_settings.version)
                    .join("metadata.json"),
                plugin_metadata,
            ) {
                error!(
                    "Something went wrong while saving the metadata.json file, error: {}",
                    err
                );
                return Err(());
            }

            let Ok(plugin_wasm) = self
                .get_file_from_registry(
                    &registry,
                    &format!("{}/{}/plugin.wasm", &name, &plugin_settings.version),
                )
                .await
            else {
                continue;
            };

            if let Err(err) = fs::write(
                directory
                    .join(&name)
                    .join(&plugin_settings.version)
                    .join("plugin.wasm"),
                plugin_wasm,
            ) {
                error!(
                    "Something went wrong while saving the plugin.wasm file, error: {}",
                    err
                );
                return Err(());
            }

            available_plugins.push(AvailablePlugin {
                name: name.clone(),
                version: plugin_settings.version.clone(),
                environment: plugin_settings.environment.clone(),
                settings: plugin_settings.settings.clone(),
            });
        }

        Ok(available_plugins)
    }
}
