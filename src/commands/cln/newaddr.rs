use std::sync::Arc;

use cln_rpc::ClnRpc;
use cln_rpc::Request::NewAddr;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use cln_rpc::model::requests::NewaddrAddresstype;
use tokio::sync::Mutex;
use super::format_json;
use crate::utils::get_option_as_string;

trait AddressString {
    fn to_string(&self) -> String;
    fn from_string(s: &str) -> Self;
}

impl AddressString for NewaddrAddresstype {
    fn to_string(&self) -> String {
        (
            match self {
                NewaddrAddresstype::BECH32 => "bech32",
                NewaddrAddresstype::P2TR => "p2tr",
                NewaddrAddresstype::ALL => "all",
                _ => "bech32",
            }
        ).to_string()
    }

    fn from_string(s: &str) -> Self {
        match s {
            "bech32" => NewaddrAddresstype::BECH32,
            "p2tr" => NewaddrAddresstype::P2TR,
            "all" => NewaddrAddresstype::ALL,
            _ => NewaddrAddresstype::BECH32,
        }
    }
}

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let addr_type = options
        .get(0)
        .and_then(|opt| get_option_as_string(opt.clone()))
        .map(|s| AddressString::from_string(&s))
        .unwrap();

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
                .name("address")
                .description("Address type")
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice(&bech32, &bech32)
                .add_string_choice(&p2tr, &p2tr)
                .add_string_choice(&all, &all)
        })
}
