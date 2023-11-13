use std::sync::Arc;

use cln_rpc::model::requests::ListpaysStatus;
use cln_rpc::primitives::Sha256;
use cln_rpc::ClnRpc;
use cln_rpc::Request::ListPays;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::commands::discord_command_options_to_map;
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let bolt11: Option<String> = get_option_as(&options_map, "bolt11");
    let payment_hash: Option<Sha256> = get_option_as(&options_map, "payment_hash");
    let status: Option<ListpaysStatus> = get_option_as(&options_map, "status");

    let req = cln_rpc::model::requests::ListpaysRequest {
        bolt11,
        payment_hash,
        status,
    };
    let res = cln_client.lock().await.call(ListPays(req)).await.unwrap();

    format_json(res)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("cln_listpays")
        .description("Get payment status")
        .create_option(|opt| {
            opt.name("bolt11")
                .description("The bolt11 invoice")
                .kind(CommandOptionType::String)
                .required(false)
        })
        .create_option(|opt| {
            opt.name("payment_hash")
                .description("The payment hash")
                .kind(CommandOptionType::String)
                .required(false)
        })
        .create_option(|opt| {
            opt.name("status")
                .description("The payment status")
                .kind(CommandOptionType::String)
                .required(false)
        })
}
