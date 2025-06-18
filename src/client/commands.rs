use std::{error::Error, sync::Arc};

use poise::{self, Command, serenity_prelude::Command as GCommand};
use tokio::sync::Mutex;

use crate::plugins::{Runtime, runtime::InitializedPlugin};

use super::Data;

pub struct Commands;

impl Commands {
    pub fn builder(
        initiated_plugins: &Vec<InitializedPlugin>,
        runtime: Arc<Mutex<Runtime>>,
    ) -> Vec<Command<Arc<Mutex<Data>>, Box<dyn Error + Send + Sync>>> {
        //for plugin in initiated_plugins {
        //    for command in plugin.events.commands {
        //        let n_command = Command::default()
        //            .create_as_slash_command()
        //            .unwrap()
        //            .name(command.name)
        //            .description(command.description)
        //            .set_options(vec![])
        //            .clone_into;

        //        //GCommand::create_global_command(cache_http, n_command).await
        //    }
        //}
        vec![]
    }
}
