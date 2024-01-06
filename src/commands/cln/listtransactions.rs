use std::sync::Arc;

use cln_rpc::ClnRpc;
use cln_rpc::Request::ListTransactions;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use crate::utils::format_json;

pub async fn run(_options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let req = cln_rpc::model::requests::ListtransactionsRequest {};

    match cln_client.lock().await.call(ListTransactions(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("cln_listtransactions")
        .description("Get the list of transactions that was stored in the wallet");

    command
}
