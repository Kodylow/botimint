use std::sync::Arc;

use cln_rpc::ClnRpc;
use fedimint_client::ClientArc;
use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::prelude::{GuildId, Message, Ready};
use serenity::prelude::{Context, EventHandler};
use tokio::sync::Mutex;
use tracing::{error, info};

use crate::fedimint_local::Fedimint;
use crate::{commands, utils};

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
    ) -> Botimint {
        Botimint {
            reqwest_client,
            cln_client,
            fm_client,
            guild_id,
        }
    }
}

#[async_trait]
impl EventHandler for Botimint {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            info!("Received command interaction: {:#?}", command.data.name);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "id" => commands::id::run(&command.data.options),
                "federation_id" => commands::id::run(&command.data.options),
                "cln_info" => {
                    commands::cln::info::run(&command.data.options, &self.cln_client).await
                }
                // "cln_connection_string" => {
                //     commands::cln::connection_string::run(&command.data.options,
                // &self.cln_client) }
                // "cln_connect" => {
                //     commands::cln::connect::run(&command.data.options, &self.cln_client)
                // }
                // "cln_balance" => {
                //     commands::cln::balance::run(&command.data.options, &self.cln_client)
                // }
                // "cln_withdraw" => {
                //     commands::cln::withdraw::run(&command.data.options, &self.cln_client)
                // }
                // "cln_deposit" => {
                //     commands::cln::deposit::run(&command.data.options, &self.cln_client)
                // }
                // "cln_pay" => commands::cln::pay::run(&command.data.options, &self.cln_client),
                // "cln_invoice" => {
                //     commands::cln::invoice::run(&command.data.options, &self.cln_client)
                // }
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

        // if let Err(_e) =
        //     utils::create_and_log_command(&ctx.http,
        // commands::wonderful_command::register).await {
        //     error!("Error creating wonderful command");
        // }
        // if let Err(_e) =
        //     utils::create_and_log_command(&ctx.http,
        // commands::numberinput::register).await {
        //     error!("Error creating numberinput command");
        // }
        // if let Err(_e) = utils::create_and_log_command(&ctx.http,
        // commands::ping::register).await {     error!("Error creating ping
        // command"); }
        if let Err(_e) =
            utils::create_and_log_command(&ctx.http, commands::cln::info::register).await
        {
            error!("Error creating cln_info command");
        }
        // if let Err(_e) =
        //     utils::create_and_log_command(&ctx.http,
        // commands::attachmentinput::register).await {
        //     error!("Error creating attachmentinput command");
        // }
        // if let Err(_e) = utils::create_and_log_command(&ctx.http,
        // commands::id::register).await {     error!("Error creating id
        // command"); }
        // if let Err(_e) = utils::create_and_log_command(&ctx.http,
        // commands::welcome::register).await {
        //     error!("Error creating welcome command");
        // }
    }
}
