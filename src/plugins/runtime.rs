use std::{collections::HashMap, fs, path::Path};

use discord_bot::plugin::plugin_types::{EventResponse, EventTypes};
use tracing::{error, info};
use wasmtime::{Config, Engine, Store, component::Linker};
use wasmtime_wasi::{
    DirPerms, FilePerms, ResourceTable,
    p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView},
};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

use crate::requests::AvailablePlugin;
wasmtime::component::bindgen!({ world: "plugin", async: true});

pub struct Runtime {
    engine: Engine,
    linker: Linker<InternalRuntime>,
    plugins: HashMap<String, (Plugin, Store<InternalRuntime>)>,
}

struct InternalRuntime {
    ctx: WasiCtx,
    http: WasiHttpCtx,
    table: ResourceTable,
}

impl WasiView for InternalRuntime {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

impl WasiHttpView for InternalRuntime {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http
    }
}

impl IoView for InternalRuntime {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

#[derive(Clone, Debug)]
pub struct InitializedPlugin {
    pub name: String,
    pub commands: Vec<InitializedPluginCommand>,
    pub message_event: bool,
    pub is_dependency: bool,
}

#[derive(Clone, Debug)]
pub struct InitializedPluginCommand {
    id: String,
    name: String,
    description: Option<String>,
}

impl Runtime {
    pub fn new() -> Self {
        // Configure the WASM engine and create an instance
        let mut config = Config::new();
        config.async_support(true);
        //// TODO: Create wasmtime epoch interuption
        //config.epoch_interruption(true);

        let engine = wasmtime::Engine::new(&config).unwrap();

        // Configure the linker, here host exports can be defined (still need to manually define
        // some functions, see the WIT file)
        //
        // Maybe there is a better way to link dependency plugins here
        //
        // Maybe there is a better way to add logging support here
        let mut linker = wasmtime::component::Linker::<InternalRuntime>::new(&engine);
        wasmtime_wasi::p2::add_to_linker_async(&mut linker).unwrap();
        wasmtime_wasi_http::add_only_http_to_linker_async(&mut linker).unwrap();

        let plugins = HashMap::new();

        Runtime {
            engine,
            linker,
            plugins,
        }
    }

    pub async fn initializing_plugins(
        &mut self,
        plugins: &Vec<AvailablePlugin>,
        directory: &Path,
    ) -> Result<Vec<InitializedPlugin>, ()> {
        let mut initialized_plugins = vec![];

        for plugin in plugins {
            let plugin_dir = directory.join(&plugin.name).join(&plugin.version);

            // Load the component from disk
            let bytes = std::fs::read(plugin_dir.join("plugin.wasm")).unwrap();
            let component = wasmtime::component::Component::new(&self.engine, bytes).unwrap();

            let env_hm = plugin.environment.clone().unwrap_or(HashMap::new());

            let env: Box<[(&str, &str)]> = env_hm
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();

            let ws_dir = plugin_dir.join("workspace");

            match fs::exists(&ws_dir) {
                Ok(exists) => match exists {
                    true => (),
                    false => {
                        if let Err(err) = fs::create_dir(&ws_dir) {
                            error!(
                                "Something went wrong while creating the workspace directory for a plugin, error: {}",
                                &err
                            );
                        }
                    }
                },
                Err(err) => {
                    error!(
                        "Something went wrong while checking if the workspace directory of a plugin exists, error: {}",
                        &err
                    );
                    return Err(());
                }
            }

            // Create an instance of the WASM store
            let mut store = wasmtime::Store::<InternalRuntime>::new(
                &self.engine,
                InternalRuntime {
                    ctx: WasiCtxBuilder::new()
                        .envs(&env)
                        .preopened_dir(
                            plugin_dir.join("workspace"),
                            "/",
                            DirPerms::all(),
                            FilePerms::all(),
                        )
                        .unwrap()
                        .build(),
                    http: WasiHttpCtx::new(),
                    table: ResourceTable::new(),
                },
            );

            // Validate and instantiate the component
            let instance = Plugin::instantiate_async(&mut store, &component, &self.linker)
                .await
                .unwrap();

            let init_result = instance
                .interface0
                .call_init(
                    &mut store,
                    &plugin
                        .settings
                        .clone()
                        .unwrap_or(serde_json::Value::default())
                        .to_string(),
                )
                .await
                .unwrap()
                .unwrap(); // Need to check properly

            info!("{:#?}", &init_result);

            let mut commands = vec![];

            for command in init_result.events.commands {
                commands.push(InitializedPluginCommand {
                    id: command.id,
                    name: command.name,
                    description: Some(command.description),
                })
            }

            initialized_plugins.push(InitializedPlugin {
                name: plugin.name.clone(),
                commands,
                message_event: init_result.events.message,
                is_dependency: init_result.is_dependency,
            });

            self.plugins.insert(plugin.name.clone(), (instance, store));
        }

        Ok(initialized_plugins)
    }

    pub async fn call_event(
        &mut self,
        plugin_name: &str,
        event: &EventTypes,
    ) -> Option<EventResponse> {
        let (plugin, store) = self.plugins.get_mut(plugin_name).unwrap();

        plugin
            .discord_bot_plugin_plugin_functions()
            .call_event(store, event)
            .await
            .unwrap()
    }
}
