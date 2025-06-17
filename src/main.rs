use std::{
    collections::VecDeque,
    env::{args_os, current_exe},
    ffi::OsString,
    os::unix::process::CommandExt,
    process::{Command, ExitCode, exit},
    sync::Arc,
    time::Duration,
};

use axum::{Router, routing::get};
use clap::Parser;
use tokio::sync::Mutex;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing::{error, info, level_filters::LevelFilter};
use tracing_appender::non_blocking::WorkerGuard;

mod cli;
use cli::Cli;
mod requests;
use requests::Requests;
mod client;
use client::Client;
mod config;
use config::Config;
mod plugins;
use plugins::Runtime;
mod utils;

fn main() -> ExitCode {
    if let Ok(should_restart) = run() {
        if !should_restart {
            info!("Exiting the bot");
            return ExitCode::from(0);
        }

        restart();
    }

    error!("Exiting the bot");
    ExitCode::from(1)
}

#[tokio::main]
async fn run() -> Result<bool, ()> {
    let cli = Cli::parse();

    let _guard = initialization(
        &cli.log_stdout_level,
        &cli.log_stdout_ansi,
        &cli.log_file_level,
        &cli.log_file_ansi,
    )?;

    info!("Loading and parsing the config file");
    let config = Config::new(&cli.config_path)?;

    info!("Creating a request client");
    let http_client = Requests::new()?;

    info!("Fetching and caching plugins");
    let available_plugins = http_client
        .fetch_plugins(&config.plugins, &config.directory, config.cache)
        .await?;

    info!("{:#?}", &available_plugins);

    info!("Creating the WASI runtime");
    let mut runtime = Runtime::new();

    info!("Initializing the plugins");
    let running_plugins = runtime
        .initializing_plugins(&available_plugins, &config.directory)
        .await
        .unwrap();

    info!("{:#?}", &running_plugins);

    info!("Creating the Discord client");
    let (mut discord_client, data) =
        Client::new(&running_plugins, Arc::new(Mutex::new(runtime))).await;

    let data_clone = data.clone();

    info!("Creating the API");
    let app = Router::new()
        .route("/", get(|| async { "Discord Bot is running" }))
        .route(
            "/logs/",
            get(|timestamp: String| async { utils::logger::read_logs(timestamp) }),
        )
        .route(
            "/handled-requests/",
            get(|| async move {
                info!("Handled requests number requested");
                format!("{}", data_clone.lock().await.handled_requests)
            }),
        )
        .route(
            "/stop/",
            get(|| async {
                shutdown().await;
                exit(0);
                ()
            }),
        )
        .route(
            "/restart/",
            get(|| async {
                shutdown().await;
                restart();
            }),
        )
        .layer((
            //TraceLayer::new_for_http(),
            //TimeoutLayer::new(Duration::from_secs(10)),
        ));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let tasks = (
        tokio::spawn(async move {
            info!("Starting the Discord client");
            discord_client.start().await
        }),
        tokio::spawn(async {
            info!("Starting the API");
            axum::serve(listener, app)
                //.with_graceful_shutdown(shutdown())
                .await
        }),
    );

    let restart = tasks.0.await.unwrap()?;

    Ok(restart)
}

fn initialization(
    log_stdout_level: &LevelFilter,
    log_stdout_ansi: &bool,
    log_file_level: &LevelFilter,
    log_file_ansi: &bool,
) -> Result<Option<WorkerGuard>, ()> {
    let guard = utils::logger::new(
        log_stdout_level,
        log_stdout_ansi,
        log_file_level,
        log_file_ansi,
    )?;

    #[cfg(feature = "dotenv")]
    {
        info!("Loading the .env file");
        utils::env::load_dotenv()?;
    }

    info!("Validating the environment variables (DISCORD_BOT_TOKEN)");
    utils::env::validate()?;

    Ok(guard)
}

async fn shutdown() {}

fn restart() {
    let bot_executable_path = match current_exe() {
        Ok(bot_executable_path) => bot_executable_path,
        Err(err) => {
            error!(
                "Failed to get the current bot executable its path: {}",
                &err
            );
            return;
        }
    };

    let mut args: VecDeque<OsString> = args_os().collect();

    args.pop_front();

    info!("Restarting the bot");
    let err = Command::new(bot_executable_path).args(args).exec();

    error!("Failed to start a new instance of the bot: {}", &err);
}
