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
    ClnNewAddr,
    ClnCreateInvoice,
    ClnFundChannel,
    ClnSendPay,
    ClnPay,
    ClnPing,
    ClnListChannels,
    ClnAddGossip,
    ClnAutoClean,
    ClnCheckMessage,
    ClnClose,
    ClnDatastore,
    ClnCreateOnion,
    ClnDelDatastore,
    Unknown,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "ping" => Self::Ping,
            "fm_federation_id" => Self::FederationId,
            "cln_info" => Self::ClnInfo,
            "cln_listpeers" => Self::ClnListPeers,
            "cln_listfunds" => Self::ClnListFunds,
            "cln_connect" => Self::ClnConnect,
            "cln_newaddr" => Self::ClnNewAddr,
            "cln_createinvoice" => Self::ClnCreateInvoice,
            "cln_fundchannel" => Self::ClnFundChannel,
            "cln_sendpay" => Self::ClnSendPay,
            "cln_pay" => Self::ClnPay,
            "cln_ping" => Self::ClnPing,
            "cln_listchannels" => Self::ClnListChannels,
            "cln_addgossip" => Self::ClnAddGossip,
            "cln_autoclean" => Self::ClnAutoClean,
            "cln_checkmessage" => Self::ClnCheckMessage,
            "cln_close" => Self::ClnClose,
            "cln_datastore" => Self::ClnDatastore,
            "cln_createonion" => Self::ClnCreateOnion,
            "cln_deldatastore" => Self::ClnDelDatastore,
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

        let commands = [
            ping::register,
            fm::id::register,
            cln::info::register,
            cln::listpeers::register,
            cln::listfunds::register,
            cln::connect::register,
            cln::newaddr::register,
            cln::createinvoice::register,
            cln::fundchannel::register,
            cln::sendpay::register,
            cln::pay::register,
            cln::ping::register,
            cln::listchannels::register,
            cln::addgossip::register,
            cln::autoclean::register,
            cln::checkmessage::register,
            cln::close::register,
            cln::datastore::register,
            cln::createonion::register,
            cln::deldatastore::register,
        ];

        for command in commands.iter() {
            create_and_log_command(&ctx.http, *command).await;
        }
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
            Command::ClnNewAddr => cln::newaddr::run(&command_data.options, &self.cln_client).await,
            Command::ClnCreateInvoice => {
                cln::createinvoice::run(&command_data.options, &self.cln_client).await
            }
            Command::ClnFundChannel => {
                cln::fundchannel::run(&command_data.options, &self.cln_client).await
            }
            Command::ClnSendPay => cln::sendpay::run(&command_data.options, &self.cln_client).await,
            Command::ClnPay => cln::pay::run(&command_data.options, &self.cln_client).await,
            Command::ClnPing => cln::ping::run(&command_data.options, &self.cln_client).await,
            Command::ClnListChannels => {
                cln::listchannels::run(&command_data.options, &self.cln_client).await
            }
            Command::ClnAddGossip => {
                cln::addgossip::run(&command_data.options, &self.cln_client).await
            }
            Command::ClnAutoClean => {
                cln::autoclean::run(&command_data.options, &self.cln_client).await
            }
            Command::ClnCheckMessage => {
                cln::checkmessage::run(&command_data.options, &self.cln_client).await
            }
            Command::ClnClose => cln::close::run(&command_data.options, &self.cln_client).await,
            Command::ClnDatastore => {
                cln::datastore::run(&command_data.options, &self.cln_client).await
            }
            Command::ClnCreateOnion => {
                cln::createonion::run(&command_data.options, &self.cln_client).await
            }
            Command::ClnDelDatastore => {
                cln::deldatastore::run(&command_data.options, &self.cln_client).await
            }
            _ => "Unknown command".to_string(),
        }
    }
}
