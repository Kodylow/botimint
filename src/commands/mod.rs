pub mod cln;
pub mod fm;
pub mod ping;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use serde_json::Value;
use serenity::model::prelude::application_command::CommandDataOption;
use serenity::model::prelude::command::CommandOptionType;

pub struct Command {
    pub register: fn(
        &mut serenity::builder::CreateApplicationCommand,
    ) -> &mut serenity::builder::CreateApplicationCommand,
    pub run: fn(
        &[CommandDataOption],
        &std::sync::Arc<tokio::sync::Mutex<cln_rpc::ClnRpc>>,
    ) -> Pin<Box<dyn Future<Output = String> + Send>>,
}

pub struct CommandOptionInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub kind: CommandOptionType,
    pub required: bool,
}

pub fn discord_command_options_to_map(
    options: &[CommandDataOption],
) -> HashMap<String, Option<Value>> {
    options
        .iter()
        .map(|opt| (opt.name.clone(), opt.value.clone()))
        .collect::<std::collections::HashMap<String, Option<Value>>>()
}
