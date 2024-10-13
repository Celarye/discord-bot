use std::process::ExitCode;

use tracing::{error, info};

mod client;
mod utils;

fn main() -> ExitCode {
    #[cfg(feature = "dotenv")]
    if let Err(exit_code) = utils::Env::load_dotenv() {
        eprintln!("Exiting the program");
        return ExitCode::from(exit_code);
    }

    let _guard;
    match utils::Logger::init() {
        Ok(pguard) => {
            if let Some(guard) = pguard {
                _guard = guard;
            }
        }
        Err(exit_code) => {
            eprintln!("Exiting the program");
            return ExitCode::from(exit_code);
        }
    }

    if let Err(exit_code) = utils::Env::validate() {
        error!("Exiting the program");
        return ExitCode::from(exit_code);
    }

    // TODO: Load, validate and parse YAML configuration

    if let Err(exit_code) = client::Base::start() {
        error!("Exiting the program");
        return ExitCode::from(exit_code);
    }

    info!("Exiting the program");
    ExitCode::from(0x0)
}
