use std::sync::Arc;

use cln_rpc::ClnRpc;
use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::prelude::{GuildId, Message, Ready};
use serenity::prelude::{Context, EventHandler};
use tokio::sync::Mutex;
use tracing::{error, info};

use crate::commands::{cln, fm, ping};
use crate::fedimint_local::Fedimint;
use crate::utils;

pub struct Botimint {
    reqwest_client: reqwest::Client,
    cln_client: Arc<Mutex<ClnRpc>>,
    fm_client: Fedimint,
    guild_id: GuildId,
}

impl Botimint {
    pub fn new(
        reqwest_client: reqwest::Client,
        cln_client: Arc<Mutex<ClnRpc>>,
        fm_client: Fedimint,
        guild_id: GuildId,
    ) -> Self {
        Self {
            reqwest_client,
            cln_client,
            fm_client,
            guild_id,
        }
    }
}

enum Command {
    Ping,
    FederationId,
    ClnInfo,
    ClnListPeers,
    ClnListFunds,
    // ClnGetConnectionString,
    ClnConnect,
    ClnNewAddr,
    Unknown,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "ping" => Self::Ping,
            "federation_id" => Self::FederationId,
            "cln_info" => Self::ClnInfo,
            "cln_listpeers" => Self::ClnListPeers,
            "cln_listfunds" => Self::ClnListFunds,
            // "cln_get_connection_string" => Self::ClnGetConnectionString,
            "cln_connect" => Self::ClnConnect,
            "cln_newaddr" => Self::ClnNewAddr,
            _ => Self::Unknown,
        }
    }
}

#[async_trait]
impl EventHandler for Botimint {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            info!("Received command interaction: {:#?}", command.data.name);

            let content = match Command::from(command.data.name.as_str()) {
                Command::Ping => ping::run(&command.data.options),
                Command::FederationId => fm::id::run(&command.data.options),
                Command::ClnInfo => cln::info::run(&command.data.options, &self.cln_client).await,
                Command::ClnListPeers => {
                    cln::listpeers::run(&command.data.options, &self.cln_client).await
                }
                Command::ClnListFunds => {
                    cln::listfunds::run(&command.data.options, &self.cln_client).await
                }
                // Command::ClnGetConnectionString => {
                //     cln::get_connection_string::run(&command.data.options,
                // &self.cln_client).await }
                Command::ClnConnect => {
                    cln::connect::run(&command.data.options, &self.cln_client).await
                }
                Command::ClnNewAddr => {
                    cln::newaddr::run(&command.data.options, &self.cln_client).await
                }
                Command::Unknown => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                error!("Cannot respond to slash command: {}", why);
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
                error!("Error sending message: {:?}", why);
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
        info!("{} is connected!", ready.user.name);
        utils::create_and_log_command(&ctx.http, ping::register).await;
        utils::create_and_log_command(&ctx.http, fm::id::register).await;
        utils::create_and_log_command(&ctx.http, cln::info::register).await;
        utils::create_and_log_command(&ctx.http, cln::listpeers::register).await;
        utils::create_and_log_command(&ctx.http, cln::listfunds::register).await;
        utils::create_and_log_command(&ctx.http, cln::connect::register).await;
        utils::create_and_log_command(&ctx.http, cln::newaddr::register).await;

        // if let Err(_e) =
        //     utils::create_and_log_command(&ctx.http,
        // cln::get_connection_string::register).await {
        //     error!("Error creating cln_get_connection_string command");
        // }
    }
}
