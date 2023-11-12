use botimint::Botimint;
use serenity::model::prelude::GuildId;
use serenity::prelude::GatewayIntents;
use serenity::Client;

use crate::fedimint_local::new_fm;
use crate::lightning::new_cln;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref CONFIG: config::Config = config::Config::from_env().unwrap();
}

mod botimint;
mod commands;
mod config;
mod fedimint_local;
mod lightning;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let cln_client = new_cln(&CONFIG.cln_rpc_path).await?;
    tracing::info!("Connected to C-Lightning RPC at {:?}", &CONFIG.cln_rpc_path);

    let reqwest_client = reqwest::Client::new();
    tracing::info!("Created new Reqwest HTTP client");

    let fm_client = new_fm().await?;
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
            reqwest_client,
            cln_client,
            fm_client,
            GuildId(CONFIG.guild_id.parse().unwrap()),
        ))
        .await
        .expect("Err creating client");
    tracing::info!("Created new Botimint client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = botimint.start().await {
        tracing::error!("Client error: {:?}", why);
    }

    Ok(())
}
