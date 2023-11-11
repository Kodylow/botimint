use std::sync::Arc;

use cln_rpc::ClnRpc;
use cln_rpc::Request::NewAddr;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use cln_rpc::model::requests::NewaddrAddresstype;
use tokio::sync::Mutex;
use crate::commands::discord_command_options_to_map;
use crate::utils::option_utils::{ get_option_as, AddressString };

use super::format_json;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let addr_type: NewaddrAddresstype = get_option_as(&options_map, "address_type").unwrap_or(
        NewaddrAddresstype::BECH32
    );

    let req = cln_rpc::model::requests::NewaddrRequest {
        addresstype: Some(addr_type),
    };
    let res = cln_client.lock().await.call(NewAddr(req)).await.unwrap();

    format_json(res)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let bech32 = AddressString::to_string(&NewaddrAddresstype::BECH32);
    let p2tr = AddressString::to_string(&NewaddrAddresstype::P2TR);
    let all = AddressString::to_string(&NewaddrAddresstype::ALL);

    command
        .name("cln_newaddr")
        .description("Get a new address for on-chain deposits to this node")
        .create_option(|option| {
            option
                .name("address_type")
                .description("Address type")
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice(&bech32, &bech32)
                .add_string_choice(&p2tr, &p2tr)
                .add_string_choice(&all, &all)
        })
}
