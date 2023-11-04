use std::env;

use anyhow::anyhow;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::command::Command;
use serenity::prelude::*;
use serenity::{async_trait, model::prelude::GuildId};
use tracing::{error, info};

mod commands;
mod utils;

struct Bot {
    client: reqwest::Client,
    guild_id: GuildId,
}

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "id" => commands::id::run(&command.data.options),
                "attachmentinput" => commands::attachmentinput::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        if let Err(e) =
            utils::create_and_log_command(&ctx.http, commands::wonderful_command::register).await
        {
            println!("Error creating wonderful command");
        }
        if let Err(e) =
            utils::create_and_log_command(&ctx.http, commands::numberinput::register).await
        {
            println!("Error creating numberinput command");
        }
        if let Err(e) = utils::create_and_log_command(&ctx.http, commands::ping::register).await {
            println!("Error creating ping command");
        }
        if let Err(e) =
            utils::create_and_log_command(&ctx.http, commands::attachmentinput::register).await
        {
            println!("Error creating attachmentinput command");
        }
        if let Err(e) = utils::create_and_log_command(&ctx.http, commands::id::register).await {
            println!("Error creating id command");
        }
        if let Err(e) = utils::create_and_log_command(&ctx.http, commands::welcome::register).await
        {
            println!("Error creating welcome command");
        }
    }
}

struct Config {
    guild_id: String,
    discord_client_token: String,
}

impl Config {
    fn from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();
        let guild_id = env::var("GUILD_ID")?;
        let discord_client_token = env::var("DISCORD_CLIENT_TOKEN")?;

        Ok(Self {
            guild_id,
            discord_client_token,
        })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::from_env().map_err(|e| anyhow!("Error reading config: {}", e))?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&config.discord_client_token, intents)
        .event_handler(Bot {
            client: reqwest::Client::new(),
            guild_id: GuildId(config.guild_id.parse().unwrap()),
        })
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(())
}
