use std::collections::HashMap;
use std::sync::Arc;

use cln_rpc::ClnRpc;
use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::prelude::application_command::CommandData;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::sync::Mutex;
use tracing::{error, info};

use crate::commands::{cln, fm, ping};
use crate::fedimint_local::Fedimint;
use crate::utils::discord_utils::create_and_log_command;

// Botimint Structure
#[allow(dead_code)]
pub struct Botimint {
    reqwest_client: reqwest::Client,
    cln_client: Arc<Mutex<ClnRpc>>,
    fm_client: Fedimint,
    guild_id: GuildId,
    command_map: HashMap<String, Command>,
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
            command_map: HashMap::new(), // Initialize with necessary commands
        }
    }
}

// Command Enum
enum Command {
    Ping,
    FederationId,
    ClnInfo,
    ClnListPeers,
    ClnListFunds,
    ClnConnect,
    // ClnNewAddr,
    ClnCreateInvoice,
    ClnFundChannel,
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
            "cln_connect" => Self::ClnConnect,
            // "cln_newaddr" => Self::ClnNewAddr,
            "cln_createinvoice" => Self::ClnCreateInvoice,
            "cln_fundchannel" => Self::ClnFundChannel,
            _ => Self::Unknown,
        }
    }
}

// EventHandler implementation for Botimint
#[async_trait]
impl EventHandler for Botimint {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            info!("Received command interaction: {:#?}", command.data.name);

            let content = self.match_command(&command.data.name, &command.data).await;

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

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                error!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
        create_and_log_command(&ctx.http, ping::register).await;
        create_and_log_command(&ctx.http, fm::id::register).await;
        create_and_log_command(&ctx.http, cln::info::register).await;
        create_and_log_command(&ctx.http, cln::listpeers::register).await;
        create_and_log_command(&ctx.http, cln::listfunds::register).await;
        create_and_log_command(&ctx.http, cln::connect::register).await;
        // create_and_log_command(&ctx.http,
        // cln::newaddr::register)     .await;
        create_and_log_command(&ctx.http, cln::createinvoice::register).await;
        create_and_log_command(&ctx.http, cln::fundchannel::register).await;
    }
}

impl Botimint {
    async fn match_command(&self, command_name: &str, command_data: &CommandData) -> String {
        match Command::from(command_name) {
            Command::Ping => ping::run(&command_data.options),
            Command::FederationId => fm::id::run(&command_data.options),
            Command::ClnInfo => cln::info::run(&command_data.options, &self.cln_client).await,
            Command::ClnListPeers => {
                cln::listpeers::run(&command_data.options, &self.cln_client).await
            }
            Command::ClnListFunds => {
                cln::listfunds::run(&command_data.options, &self.cln_client).await
            }
            Command::ClnConnect => cln::connect::run(&command_data.options, &self.cln_client).await,
            // Command::ClnNewAddr => cln::newaddr::run(&command_data.options,
            // &self.cln_client).await,
            Command::ClnCreateInvoice => {
                cln::createinvoice::run(&command_data.options, &self.cln_client).await
            }
            Command::ClnFundChannel => {
                cln::fundchannel::run(&command_data.options, &self.cln_client).await
            }
            Command::Unknown => "not implemented :(".to_string(),
        }
    }
}
