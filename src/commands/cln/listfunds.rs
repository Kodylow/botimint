use std::str::FromStr;
use std::sync::Arc;

use cln_rpc::primitives::PublicKey;
use cln_rpc::ClnRpc;
use cln_rpc::Request::ListFunds;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::utils::get_option_as_bool;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let spent = options
        .get(0)
        .and_then(|opt| get_option_as_bool(opt.clone()))
        .unwrap_or(false);
    let req = cln_rpc::model::requests::ListfundsRequest { spent: Some(spent) };
    let res = cln_client.lock().await.call(ListFunds(req)).await.unwrap();

    format_json(res)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("cln_listfunds")
        .description("Get funds info")
        .create_option(|opt| {
            opt.name("spent")
                .description("The ID of the peer")
                .kind(CommandOptionType::Boolean)
                .required(false)
        })
}
