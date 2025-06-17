use std::{env, sync::Arc};

use poise::{
    self,
    serenity_prelude::{self as serenity, Context},
};
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

use crate::plugins::{
    Runtime,
    runtime::{InitializedPlugin, discord_bot::plugin::plugin_types::EventTypes},
};

use super::{Commands, Data};

pub struct Client {
    client: serenity::Client,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
//type Context<'a> = poise::Context<'a, Data, Error>;

impl Client {
    pub async fn new(
        initialized_plugins: &Vec<InitializedPlugin>,
        runtime: Arc<Mutex<Runtime>>,
    ) -> (Self, Arc<Mutex<Data>>) {
        let token = env::var("DISCORD_BOT_TOKEN").unwrap();

        let intents =
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

        let runtime_clone = runtime.clone();
        let init_plugins = initialized_plugins.clone();
        let data = Arc::new(Mutex::new(Data {
            restart: false,
            handled_requests: 0,
            runtime: runtime_clone,
            initialized_plugins: init_plugins.clone(),
        }));

        let data_clone = data.clone();

        let framework = poise::Framework::<Arc<Mutex<Data>>, _>::builder()
            .options(poise::FrameworkOptions {
                event_handler: |ctx, event, framework, data| {
                    Box::pin(Self::event_handler(ctx, event, framework, data))
                },
                commands: Commands::builder(&init_plugins, runtime.clone()),
                ..Default::default()
            })
            .setup(|ctx, _ready, framework| {
                Box::pin(async move {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                    Ok(data_clone)
                })
            })
            .build();

        let client = serenity::ClientBuilder::new(token, intents)
            .framework(framework)
            .await
            .unwrap();

        (Client { client }, data)
    }

    pub async fn start(&mut self) -> Result<(), ()> {
        match self.client.start().await {
            Ok(()) => Ok(()),
            Err(err) => {
                error!("An error occured while starting the client: {}", &err);
                Err(())
            }
        }
    }

    async fn event_handler(
        ctx: &Context,
        event: &serenity::FullEvent,
        _framework: poise::FrameworkContext<'_, Arc<Mutex<Data>>, Error>,
        data: &Arc<Mutex<Data>>,
    ) -> Result<(), Error> {
        match event {
            serenity::FullEvent::Ready { data_about_bot, .. } => {
                info!("Logged in as {}", data_about_bot.user.name)
            }
            serenity::FullEvent::Message { new_message } => {
                if new_message.author.id == ctx.cache.current_user().id {
                    return Ok(());
                }

                let mut ldata = data.lock().await;
                for plugin in ldata.initialized_plugins.clone().iter() {
                    if !plugin.message_event {
                        continue;
                    }

                    ldata.handled_requests += 1;

                    debug!("Plugin function call: \"{}\"", plugin.name);
                    let pevent_response = ldata
                        .runtime
                        .lock()
                        .await
                        .call_event(
                            &plugin.name,
                            &EventTypes::Message(new_message.content.clone()),
                        )
                        .await;

                    if pevent_response.is_none() {
                        warn!("No response");
                        continue;
                    }

                    let event_response = pevent_response.unwrap();

                    let mut reply = poise::CreateReply::default();

                    if event_response.content.is_some() {
                        reply = reply.content(event_response.content.clone().unwrap());
                        new_message
                            .reply(ctx, event_response.content.unwrap())
                            .await?;
                    } else {
                        warn!("No content");
                    }

                    //for embed in event_response.embeds {
                    //    let mut sembed = CreateEmbed::new();

                    //    if embed.clone().author.is_some() {
                    //        let mut author = CreateEmbedAuthor::new("");

                    //        if embed.clone().author.clone().unwrap().name.is_some() {
                    //            author = author.name(embed.author.clone().unwrap().name.unwrap());
                    //        }

                    //        sembed = sembed.author(author);
                    //    }

                    //    reply = reply.embed(sembed);
                    //}

                    reply = reply.ephemeral(event_response.ephermal.unwrap_or(false));

                    reply = reply.reply(event_response.reply.unwrap_or(true));
                }
            }
            _ => {}
        }

        Ok(())
    }
}
