use std::sync::Arc;

use cln_rpc::ClnRpc;
use cln_rpc::Request::NewAddr;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::utils::option_utils::get_option_as_string;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let addr_type = options
        .get(0)
        .and_then(|opt| get_option_as_string(opt.clone()))
        .map(|s| match s.as_str() {
            "bech32" => cln_rpc::model::requests::NewaddrAddresstype::BECH32,
            "p2tr" => cln_rpc::model::requests::NewaddrAddresstype::P2TR,
            "all" => cln_rpc::model::requests::NewaddrAddresstype::ALL,
            _ => cln_rpc::model::requests::NewaddrAddresstype::BECH32,
        })
        .unwrap_or(cln_rpc::model::requests::NewaddrAddresstype::BECH32);

    let req = cln_rpc::model::requests::NewaddrRequest {
        addresstype: Some(addr_type),
    };
    let res = cln_client.lock().await.call(NewAddr(req)).await.unwrap();

    format_json(res)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("cln_newaddr")
        .description("Get a new address for on-chain deposits to this node")
        .create_option(|opt| {
            opt.name("addresstype")
                .description("The type of address to generate: bech32, p2sh-segwit, or all")
                .kind(CommandOptionType::String)
                .required(false)
        })
}
