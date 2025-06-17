use std::{error::Error, sync::Arc};

use poise::{self, Command};
use tokio::sync::Mutex;

use crate::plugins::{Runtime, runtime::InitializedPlugin};

use super::Data;

pub struct Commands;

impl Commands {
    pub fn builder(
        initiated_plugins: &Vec<InitializedPlugin>,
        runtime: Arc<Mutex<Runtime>>,
    ) -> Vec<Command<Arc<Mutex<Data>>, Box<dyn Error + Send + Sync>>> {
        let commands = vec![];

        // TODO: build

        commands
    }
}
