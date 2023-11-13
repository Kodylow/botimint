use std::sync::Arc;

use cln_rpc::model::requests::ListforwardsStatus;
use cln_rpc::primitives::ShortChannelId;
use cln_rpc::ClnRpc;
use cln_rpc::Request::ListForwards;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::commands::discord_command_options_to_map;
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let status: Option<ListforwardsStatus> = get_option_as(&options_map, "status");
    let in_channel: Option<ShortChannelId> = get_option_as(&options_map, "in_channel");
    let out_channel: Option<ShortChannelId> = get_option_as(&options_map, "out_channel");

    let req = cln_rpc::model::requests::ListforwardsRequest {
        status,
        in_channel,
        out_channel,
    };
    let res = cln_client
        .lock()
        .await
        .call(ListForwards(req))
        .await
        .unwrap();

    format_json(res)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("cln_listforwards")
        .description("Get information about all HTLCs")
        .create_option(|opt| {
            opt.name("status")
                .description("The status of the forwards")
                .kind(CommandOptionType::String)
                .required(false)
        })
        .create_option(|opt| {
            opt.name("in_channel")
                .description("The incoming channel")
                .kind(CommandOptionType::String)
                .required(false)
        })
        .create_option(|opt| {
            opt.name("out_channel")
                .description("The outgoing channel")
                .kind(CommandOptionType::String)
                .required(false)
        })
}
