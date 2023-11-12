use std::sync::Arc;

use cln_rpc::ClnRpc;
use fedimint_client::ClientArc;
use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::sync::Mutex;
use tracing::{error, info};

use crate::commands::{cln, fm, ping};
use crate::utils::discord_utils::create_and_log_command;

// Botimint Structure
#[allow(dead_code)]
pub struct Botimint {
    reqwest_client: reqwest::Client,
    cln_client: Arc<Mutex<ClnRpc>>,
    fm_client: ClientArc,
    guild_id: GuildId,
}

impl Botimint {
    pub fn new(
        reqwest_client: reqwest::Client,
        cln_client: Arc<Mutex<ClnRpc>>,
        fm_client: ClientArc,
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

// EventHandler implementation for Botimint
#[async_trait]
impl EventHandler for Botimint {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            info!("Received command interaction: {:#?}", command.data.name);

            let content = match command.data.name.as_str() {
                name if name.starts_with("cln_") => {
                    cln::handle_run(name, &command.data, &self.cln_client).await
                }
                name if name.starts_with("fm_") => {
                    fm::handle_run(name, &command.data, &self.fm_client).await
                }
                _ => ping::run(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                error!("Cannot respond to slash command: {:?}", why);
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

        cln::ready(&ctx).await;
        fm::ready(&ctx).await;
        // get rid of this once we have more commands
        create_and_log_command(&ctx.http, ping::register).await;
    }
}
