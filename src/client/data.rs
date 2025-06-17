use std::sync::Arc;

use tokio::sync::Mutex;

use crate::plugins::{Runtime, runtime::InitializedPlugin};

pub struct Data {
    pub restart: bool,
    pub handled_requests: u32,
    pub runtime: Arc<Mutex<Runtime>>,
    pub initialized_plugins: Vec<InitializedPlugin>,
}
