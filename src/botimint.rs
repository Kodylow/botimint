use std::sync::Arc;

use cln_rpc::ClnRpc;
use fedimint_client::ClientArc;
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use tokio::sync::Mutex;
use tracing::{error, info};

use crate::commands::{cln, custom, fed};

// Botimint Structure
pub struct Botimint {
    cln_client: Arc<Mutex<ClnRpc>>,
    fm_client: ClientArc,
}

impl Botimint {
    pub fn new(cln_client: Arc<Mutex<ClnRpc>>, fm_client: ClientArc) -> Self {
        Self {
            cln_client,
            fm_client,
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
                    fed::handle_run(name, &command.data, &self.fm_client).await
                }
                _ => custom::handle_run(&command.data.name, &command.data).await,
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
        fed::ready(&ctx).await;
        custom::ready(&ctx).await;
    }
}
