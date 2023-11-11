use std::str::FromStr;
use std::sync::Arc;

use cln_rpc::primitives::PublicKey;
use cln_rpc::ClnRpc;
use cln_rpc::Request::ListPeers;
use serde_json::json;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::utils::get_option_as_string;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let mut id = None;
    let mut level = None;
    for option in options {
        match option.name.as_str() {
            "id" => {
                id = match get_option_as_string(option.clone()) {
                    Some(s) => Some(PublicKey::from_str(&s).unwrap()),
                    None => None,
                };
            }
            "level" => {
                level = get_option_as_string(option.clone());
            }
            _ => {}
        }
    }
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
