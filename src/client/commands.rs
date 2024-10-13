use poise::{self, serenity_prelude as serenity};

use crate::client;
pub struct Commands;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, client::Data, Error>;

impl Commands {
    #[poise::command(slash_command, prefix_command)]
    pub async fn status(ctx: Context<'_>) -> Result<(), Error> {
        let embed = poise::CreateReply::default()
            .embed(
                serenity::CreateEmbed::new()
                    .title("Client Status Message")
                    .description(
                    "This is the client status message, here you can find the client its statuses!",
                ),
            )
            .ephemeral(true);
        ctx.send(embed).await?;
        Ok(())
    }

    #[poise::command(slash_command, prefix_command)]
    pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
        let embed = poise::CreateReply::default()
            .embed(
                serenity::CreateEmbed::new()
                    .title("Client Help Message")
                    .description(
                        "This is the client help message, here you can find help for all things in regard to the client!",
                    ),
            )
            .ephemeral(true);
        ctx.send(embed).await?;
        Ok(())
    }

    pub fn list() -> Vec<
        poise::Command<
            client::Data,
            Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>,
        >,
    > {
        vec![Self::status(), Self::help()]
    }
}
