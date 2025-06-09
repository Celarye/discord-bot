use std::env;

use poise::serenity_prelude::validate_token;
use tracing::{debug, error};

#[cfg(feature = "dotenv")]
use dotenvy::dotenv;

#[cfg(feature = "dotenv")]
pub fn load_dotenv() -> Result<(), ()> {
    if let Err(err) = dotenv() {
        error!(
            "An error occurred wile trying to load the .env file: {}",
            &err
        );
        return Err(());
    }

    Ok(())
}

pub fn validate() -> Result<(), ()> {
    if let Ok(value) = env::var("DISCORD_BOT_TOKEN") {
        if let Err(err) = validate_token(&value) {
            error!(
                "DISCORD_BOT_TOKEN environment variable was of the incorrect structure: {}",
                &err
            );
            return Err(());
        }

        debug!(
            "DISCORD_BOT_TOKEN environment variable was found and was of the correct structure: {:.3}... (redacted)",
            &value
        );
    } else {
        error!(
            "The DISCORD_BOT_TOKEN environment variable was not set, contains an illegal character ('=' or '0') or was not valid UNICODE"
        );
        return Err(());
    }

    Ok(())
}
