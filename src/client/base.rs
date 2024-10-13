use poise::{self, serenity_prelude as serenity};
use tokio;
use tracing::{debug, error};

use crate::client;
pub struct Base;

impl Base {
    #[tokio::main]
    pub async fn start() -> Result<(), u8> {
        let token;
        match std::env::var("DISCORD_TOKEN") {
            Ok(value) => match serenity::validate_token(&value) {
                Ok(()) => {
                    debug!(
                        "DISCORD_TOKEN environment variable was found: {:.3}... (redacted)",
                        &value
                    );
                    token = value
                }
                Err(err) => {
                    error!(
                        "An error occurred wile trying to load the Discord token: {}",
                        &err
                    );
                    return Err(0x1);
                }
            },
            Err(err) => {
                error!(
                    "An error occurred wile trying to load the Discord token: {}",
                    &err
                );
                return Err(0x1);
            }
        }

        let intents = serenity::GatewayIntents::non_privileged();

        let framework = poise::Framework::builder()
            .options(poise::FrameworkOptions {
                commands: client::Commands::list(),
                ..Default::default()
            })
            .setup(|ctx, _ready, framework| {
                Box::pin(async move {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    Ok(client::Data {})
                })
            })
            .build();

        let pclient = serenity::ClientBuilder::new(token, intents)
            .framework(framework)
            .await;
        match pclient {
            Ok(mut client) => match client.start().await {
                Ok(()) => (),
                Err(err) => {
                    error!("An error occured while starting the client: {}", &err);
                    return Err(0x1);
                }
            },
            Err(err) => {
                error!("An error occured while starting the client: {}", &err);
                return Err(0x1);
            }
        }

        Ok(())
    }
}
