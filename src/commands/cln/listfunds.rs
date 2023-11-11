use std::sync::Arc;

use cln_rpc::ClnRpc;
use cln_rpc::Request::ListFunds;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::utils::option_utils::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = super::discord_command_options_to_map(options);
    let spent: bool = get_option_as(&options_map, "spent").unwrap_or(false);
    let req = cln_rpc::model::requests::ListfundsRequest { spent: Some(spent) };
    let res = cln_client.lock().await.call(ListFunds(req)).await.unwrap();

    format_json(res)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let option_info = CommandOptionInfo {
        name: "spent",
        description: "The ID of the peer",
        kind: CommandOptionType::Boolean,
        required: false,
    };

    command
        .name("cln_listfunds")
        .description("Get funds info")
        .create_option(|opt| {
            opt.name(option_info.name)
                .description(option_info.description)
                .kind(option_info.kind)
                .required(option_info.required)
        })
}
