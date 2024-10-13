use std::env;

use tracing::{info, level_filters::LevelFilter};
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Layer, Registry};

pub struct Logger;

impl Logger {
    pub fn init() -> Result<Option<WorkerGuard>, u8> {
        let log_stdout_level =
            &(*env::var("LOG_STDOUT_LEVEL").unwrap_or_else(|_| "INFO".to_string()));

        let log_files_level =
            &(*env::var("LOG_FILES_LEVEL").unwrap_or_else(|_| "INFO".to_string()));

        match log_stdout_level {
            "OFF" => match log_files_level {
                "OFF" => Ok(None),
                _ => {
                    // TODO: Make the file logger configurable
                    let prolling_file_appender = RollingFileAppender::builder()
                        .rotation(Rotation::DAILY)
                        .filename_prefix("pbc-discord-bot")
                        .filename_suffix("log")
                        .max_log_files(7)
                        .build("logs");

                    match prolling_file_appender {
                        Ok(rolling_file_appender) => {
                            let file_env_filter = EnvFilter::builder()
                                .with_default_directive(LevelFilter::INFO.into())
                                .with_env_var("LOG_FILES_LEVEL")
                                .from_env_lossy();

                            let (non_blocking, guard) =
                                tracing_appender::non_blocking(rolling_file_appender);

                            let subscriber = Registry::default().with(
                                fmt::Layer::default()
                                    .with_writer(non_blocking)
                                    .with_ansi(false)
                                    .with_filter(file_env_filter),
                            );

                            match tracing::subscriber::set_global_default(subscriber) {
                                Ok(()) => {
                                    info!("Logger has been initialized");
                                    Ok(Some(guard))
                                }
                                Err(err) => {
                                    eprintln!(
                                        "An error occurred while initializing the logger: {}",
                                        &err
                                    );
                                    Err(0x1)
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("An error occurred while initializing the logger: {}", &err);
                            Err(0x1)
                        }
                    }
                }
            },
            _ => match log_files_level {
                "OFF" => {
                    let stdout_env_filter = EnvFilter::builder()
                        .with_default_directive(LevelFilter::INFO.into())
                        .with_env_var("LOG_STDOUT_LEVEL")
                        .from_env_lossy();

                    let subscriber = Registry::default()
                        // TODO: Make the ansi option configurable
                        .with(
                            fmt::Layer::default()
                                .with_writer(std::io::stdout)
                                .with_ansi(true)
                                .with_filter(stdout_env_filter),
                        );

                    match tracing::subscriber::set_global_default(subscriber) {
                        Ok(()) => {
                            info!("Logger has been initialized");
                            Ok(None)
                        }
                        Err(err) => {
                            eprintln!("An error occurred while initializing the logger: {}", &err);
                            Err(0x1)
                        }
                    }
                }
                _ => {
                    // TODO: Make the file logger configurable
                    let prolling_file_appender = RollingFileAppender::builder()
                        .rotation(Rotation::DAILY)
                        .filename_prefix("pbc-discord-bot")
                        .filename_suffix("log")
                        .max_log_files(7)
                        .build("logs");

                    match prolling_file_appender {
                        Ok(rolling_file_appender) => {
                            let stdout_env_filter = EnvFilter::builder()
                                .with_default_directive(LevelFilter::INFO.into())
                                .with_env_var("LOG_STDOUT_LEVEL")
                                .from_env_lossy();

                            let file_env_filter = EnvFilter::builder()
                                .with_default_directive(LevelFilter::INFO.into())
                                .with_env_var("LOG_FILES_LEVEL")
                                .from_env_lossy();

                            let (non_blocking, guard) =
                                tracing_appender::non_blocking(rolling_file_appender);

                            let subscriber = Registry::default()
                                // TODO: Make the ansi option configurable
                                .with(
                                    fmt::Layer::default()
                                        .with_writer(std::io::stdout)
                                        .with_ansi(true)
                                        .with_filter(stdout_env_filter),
                                )
                                .with(
                                    fmt::Layer::default()
                                        .with_writer(non_blocking)
                                        .with_ansi(false)
                                        .with_filter(file_env_filter),
                                );

                            match tracing::subscriber::set_global_default(subscriber) {
                                Ok(()) => {
                                    info!("Logger has been initialized");
                                    Ok(Some(guard))
                                }
                                Err(err) => {
                                    eprintln!(
                                        "An error occurred while initializing the logger: {}",
                                        &err
                                    );
                                    Err(0x1)
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("An error occurred while initializing the logger: {}", &err);
                            Err(0x1)
                        }
                    }
                }
            },
        }
    }
}
