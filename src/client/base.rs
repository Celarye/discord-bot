use std::{
    env,
    sync::{Arc, atomic::AtomicU32},
};

use poise::{
    self, SlashArgument,
    serenity_prelude::{
        self as serenity, Attachment, CacheHttp, Context, CreateCommandOption, prelude::TypeMapKey,
    },
};
use tokio::sync::Mutex;
use tracing::{debug, error, field::debug, info};

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

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Data;
}

impl Client {
    pub async fn new(
        initialized_plugins: &Vec<InitializedPlugin>,
        runtime: Arc<Mutex<Runtime>>,
    ) -> Self {
        let token = env::var("DISCORD_BOT_TOKEN").unwrap();

        let intents = serenity::GatewayIntents::non_privileged();

        let runtime_clone = runtime.clone();
        let init_plugins = initialized_plugins.clone();

        let framework = poise::Framework::builder()
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

                    Ok(Data {
                        restart: false,
                        handled_requests: AtomicU32::new(0),
                        runtime: runtime_clone,
                        initialized_plugins: init_plugins.clone(),
                    })
                })
            })
            .build();

        let client = serenity::ClientBuilder::new(token, intents)
            .framework(framework)
            .await
            .unwrap();

        // messy
        client
            .data
            .write()
            .await
            .insert::<ShardManagerContainer>(Data {
                restart: false,
                handled_requests: AtomicU32::new(0),
                runtime: runtime.clone(),
                initialized_plugins: initialized_plugins.clone(),
            });

        Client { client }
    }

    pub async fn start(&mut self) -> Result<bool, ()> {
        match self.client.start().await {
            Ok(()) => Ok(self
                .client
                .data
                .read()
                .await
                .get::<ShardManagerContainer>()
                .unwrap()
                .restart),
            Err(err) => {
                error!("An error occured while starting the client: {}", &err);
                Err(())
            }
        }
    }

    async fn event_handler(
        ctx: &Context,
        event: &serenity::FullEvent,
        _framework: poise::FrameworkContext<'_, Data, Error>,
        data: &Data,
    ) -> Result<(), Error> {
        match event {
            serenity::FullEvent::Ready { data_about_bot, .. } => {
                info!("Logged in as {}", data_about_bot.user.name)
            }
            serenity::FullEvent::Message { new_message } => {
                info!("Message event");
                info!("Message content: {}", &new_message.content);
                for plugin in data.initialized_plugins.iter() {
                    info!("Plugin: {:#?}", plugin);
                    if !plugin.message_event {
                        continue;
                    }

                    data.handled_requests
                        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

                    info!("Plugin function call: \"{}\"", plugin.name);
                    let pevent_response = data
                        .runtime
                        .lock()
                        .await
                        .call_event(
                            &plugin.name,
                            &EventTypes::Message(new_message.content.clone()),
                        )
                        .await;

                    if pevent_response.is_none() {
                        info!("No response");
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
                        info!("No content");
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
