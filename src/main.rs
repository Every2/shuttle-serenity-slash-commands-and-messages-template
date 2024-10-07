use anyhow::Context as _;
use chibiana::commands;
use chrono::{Datelike, TimeZone};
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use tokio::time;
use tracing::info;

struct Handler {
    discord_token: String,
    guild_id: String,
    channel_id: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Interactions: {command:#?}");

            let content = match command.data.name.as_str() {
                "foo" => Some(commands::foo::run(&command.data.options())),
                "too" => Some(commands::too::run(&command.data.options())),
                _ => Some("Invalid command... :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("I can't answer this interaction: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is online", ready.user.name);

        let guild_id = GuildId::new(self.guild_id.parse().expect("GUILD ID INT"));

        let commands = guild_id
            .set_commands(
                &ctx.http,
                vec![commands::foo::register(), commands::too::register()],
            )
            .await;

        println!("I have these commands: {commands:#?}");
    }
}


#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {

    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let channel_id = secret_store
        .get("CHANNELID")
        .context("'CHANNEL_ID' was not found")?;

    let discord_guild_id = secret_store
        .get("GUILD_ID")
        .context("'DISCORD_GUILD_ID' was not found")?;

    let s = secret_store.clone();
    let d = secret_store.clone();
    let c = secret_store.clone();

    let mut client = get_client(
        &discord_token,
        &channel_id,
        discord_guild_id.parse().unwrap(),
    )
    .await;

    tokio::spawn(async move {
        loop {
            let discord_token = s
                .get("DISCORD_TOKEN")
                .context("'DISCORD_TOKEN' was not found").unwrap();

            let channel_id = s
                .get("CHANNELID")
                .context("'CHANNEL_ID' was not found").unwrap();

            let discord_guild_id = s
                .get("GUILD_ID")
                .context("'DISCORD_GUILD_ID' was not found").unwrap();

            let mut client = get_client(
                &discord_token,
                &channel_id,
                discord_guild_id.parse().unwrap(),
            )
            .await;
            let now = chrono::Local::now();
            let target_time = chrono::Local
                .with_ymd_and_hms(now.year(), now.month(), now.day(), 02, 00, 00)
                .unwrap();
            let target_time = if target_time < now {
                target_time + chrono::Duration::days(1)
            } else {
                target_time
            };

            let duration = (target_time - now).to_std().unwrap();
            time::sleep(duration).await;

            ChannelId::new(channel_id.parse::<u64>().unwrap())
                .say(
                    &client.http,
                    "A message!",
                )
                .await
                .unwrap();

            client.start().await.unwrap();
        }
    });

    tokio::spawn(async move {
        loop {
            let discord_token = d
                .get("DISCORD_TOKEN")
                .context("'DISCORD_TOKEN' was not found").unwrap();

            let channel_id = d
                .get("CHANNELID")
                .context("'CHANNEL_ID' was not found").unwrap();

            let discord_guild_id = d
                .get("GUILD_ID")
                .context("'DISCORD_GUILD_ID' was not found").unwrap();

            let mut client = get_client(
                &discord_token,
                &channel_id,
                discord_guild_id.parse().unwrap(),
            )
            .await;
            let now = chrono::Local::now();
            let target_time = chrono::Local
                .with_ymd_and_hms(now.year(), now.month(), now.day(), 15, 00, 00)
                .unwrap();
            let target_time = if target_time < now {
                target_time + chrono::Duration::days(1)
            } else {
                target_time
            };

            let duration = (target_time - now).to_std().unwrap();
            time::sleep(duration).await;

            ChannelId::new(channel_id.parse::<u64>().unwrap())
                .say(
                    &client.http,
                    "other message!",
                )
                .await
                .unwrap();

            client.start().await.unwrap();
        }
    });

    tokio::spawn(async move {
        loop {
            let discord_token = c
                .get("DISCORD_TOKEN")
                .context("'DISCORD_TOKEN' was not found").unwrap();

            let channel_id = c
                .get("CHANNELID")
                .context("'CHANNEL_ID' was not found").unwrap();

            let discord_guild_id = c
                .get("GUILD_ID")
                .context("'DISCORD_GUILD_ID' was not found").unwrap();

            let mut client = get_client(
                &discord_token,
                &channel_id,
                discord_guild_id.parse().unwrap(),
            )
            .await;
            let now = chrono::Local::now();
            let target_time = chrono::Local
                .with_ymd_and_hms(now.year(), now.month(), now.day(), 11, 00, 00)
                .unwrap();
            let target_time = if target_time < now {
                target_time + chrono::Duration::days(1)
            } else {
                target_time
            };

            let duration = (target_time - now).to_std().unwrap();
            time::sleep(duration).await;

            ChannelId::new(channel_id.parse::<u64>().unwrap())
                .say(
                    &client.http,
                    "Another.",
                )
                .await
                .unwrap();

            client.start().await.unwrap();
        }
    });


    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }

    Ok(client.into())
}

pub async fn get_client(discord_token: &str, channel_id: &str, discord_guild_id: u64) -> Client {
    let intents = GatewayIntents::empty();

    Client::builder(discord_token, intents)
        .event_handler(Handler {
            channel_id: String::new(),
            discord_token: String::new(),
            guild_id: GuildId::new(discord_guild_id).to_string(),
        })
        .await
        .expect("Err creating client")
}
