use botimint::Botimint;
// use serenity::model::prelude::GuildId;
use serenity::prelude::GatewayIntents;
use serenity::Client;

use crate::config::CONFIG;
use crate::lightning::new_cln;
use crate::state::load_fedimint_client;

mod botimint;
mod commands;
mod config;
mod lightning;
mod state;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let cln_client = new_cln(&CONFIG.cln_rpc_path).await?;
    tracing::info!("Connected to C-Lightning RPC at {:?}", &CONFIG.cln_rpc_path);

    // let reqwest_client = reqwest::Client::new();
    // tracing::info!("Created new Reqwest HTTP client");

    let fm_client = load_fedimint_client().await?;
    tracing::info!("Connected to Fedimint: {:?}", fm_client.federation_id());

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Botimint Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut botimint = Client::builder(&CONFIG.discord_client_token, intents)
        .event_handler(Botimint::new(
            cln_client, fm_client,
            // GuildId(CONFIG.guild_id.parse().unwrap()),
        ))
        .await
        .expect("Err creating client");
    tracing::info!("Created new Botimint client");

    if let Err(why) = botimint.start().await {
        tracing::error!("Client error: {:?}", why);
    }

    Ok(())
}
