use std::{
    fmt::format,
    fs,
    path::{self, Path},
};

use chrono::{Date, Local};
use tracing::{info, level_filters::LevelFilter};
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{Layer, Registry, fmt, layer::SubscriberExt};

pub fn new(
    log_stdout_level: &LevelFilter,
    log_stdout_ansi: &bool,
    log_file_level: &LevelFilter,
    log_file_ansi: &bool,
) -> Result<Option<WorkerGuard>, ()> {
    if log_stdout_level != &LevelFilter::OFF {
        println!("Initializing the logger");
    }

    match log_stdout_level {
        &LevelFilter::OFF => match log_file_level {
            &LevelFilter::OFF => Ok(None),
            _ => {
                // TODO: Make the file logger configurable
                let p_rolling_file_appender = RollingFileAppender::builder()
                    .rotation(Rotation::DAILY)
                    .filename_prefix("discord-bot")
                    .filename_suffix("log")
                    .max_log_files(7)
                    .build("logs");

                match p_rolling_file_appender {
                    Ok(rolling_file_appender) => {
                        let (non_blocking, guard) =
                            tracing_appender::non_blocking(rolling_file_appender);

                        let subscriber = Registry::default().with(
                            fmt::Layer::default()
                                .with_writer(non_blocking)
                                .with_ansi(*log_stdout_ansi)
                                .with_filter(*log_file_level),
                        );

                        match tracing::subscriber::set_global_default(subscriber) {
                            Ok(()) => Ok(Some(guard)),
                            Err(err) => {
                                eprintln!(
                                    "An error occurred while initializing the logger: {}",
                                    &err
                                );
                                Err(())
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("An error occurred while initializing the logger: {}", &err);
                        Err(())
                    }
                }
            }
        },
        _ => match log_file_level {
            &LevelFilter::OFF => {
                let subscriber = Registry::default().with(
                    fmt::Layer::default()
                        .with_writer(std::io::stdout)
                        .with_ansi(*log_stdout_ansi)
                        .with_filter(*log_stdout_level),
                );

                match tracing::subscriber::set_global_default(subscriber) {
                    Ok(()) => Ok(None),
                    Err(err) => {
                        eprintln!("An error occurred while initializing the logger: {}", &err);
                        Err(())
                    }
                }
            }
            _ => {
                // TODO: Make the file logger configurable
                let prolling_file_appender = RollingFileAppender::builder()
                    .rotation(Rotation::DAILY)
                    .filename_prefix("discord-bot")
                    .filename_suffix("log")
                    .max_log_files(7)
                    .build("logs");

                match prolling_file_appender {
                    Ok(rolling_file_appender) => {
                        let (non_blocking, guard) =
                            tracing_appender::non_blocking(rolling_file_appender);

                        let subscriber = Registry::default()
                            .with(
                                fmt::Layer::default()
                                    .with_writer(std::io::stdout)
                                    .with_ansi(*log_stdout_ansi)
                                    .with_filter(*log_stdout_level),
                            )
                            .with(
                                fmt::Layer::default()
                                    .with_writer(non_blocking)
                                    .with_ansi(*log_file_ansi)
                                    .with_filter(*log_file_level),
                            );

                        match tracing::subscriber::set_global_default(subscriber) {
                            Ok(()) => Ok(Some(guard)),
                            Err(err) => {
                                eprintln!(
                                    "An error occurred while initializing the logger: {}",
                                    &err
                                );
                                Err(())
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("An error occurred while initializing the logger: {}", &err);
                        Err(())
                    }
                }
            }
        },
    }
}

pub fn read_logs(timestamp: String) -> String {
    let log_path = Path::new("logs").join(format!(
        "discord-bot.{}.log",
        Local::now().date_naive().format("%Y-%m-%d")
    ));
    let logs = fs::read_to_string(log_path).unwrap_or(String::from(""));

    logs
}
