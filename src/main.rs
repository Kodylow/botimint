



use botimint::Botimint;

use fedimint_local::Fedimint;
use lightning::Cln;



use serenity::model::prelude::GuildId;
use serenity::prelude::{GatewayIntents};
use serenity::{Client};


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
    tracing_subscriber::fmt::init();

    let cln_client = Cln::new(&CONFIG.cln_rpc_path).await?;

    let reqwest_client = reqwest::Client::new();

    let fm_client = Fedimint::new().await?;

    println!(
        "Connected to Fedimint: {:?}",
        fm_client.client.federation_id()
    );

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
            GuildId(CONFIG.guild_id.parse().unwrap()),
        ))
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = botimint.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(())
}
