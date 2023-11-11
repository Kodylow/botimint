use std::sync::Arc;

use cln_rpc::primitives::PublicKey;
use cln_rpc::ClnRpc;
use cln_rpc::Request::ListPeers;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::commands::discord_command_options_to_map;
use crate::utils::option_utils::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let id: Option<PublicKey> = get_option_as(&options_map, "id");
    let level: Option<String> = get_option_as(&options_map, "level");

    let req = cln_rpc::model::requests::ListpeersRequest { id, level };
    let res = cln_client.lock().await.call(ListPeers(req)).await.unwrap();

    format_json(res)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("cln_listpeers")
        .description("Get peer info")
        .create_option(|opt| {
            opt.name("id")
                .description("The ID of the peer")
                .kind(CommandOptionType::String)
                .required(false)
        })
        .create_option(|opt| {
            opt.name("level")
                .description("The level of detail")
                .kind(CommandOptionType::String)
                .required(false)
        })
}
