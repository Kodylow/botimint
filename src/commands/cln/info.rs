use std::sync::Arc;

use cln_rpc::ClnRpc;
use cln_rpc::Request::Getinfo;
use serde_json::json;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

pub async fn run(_options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let req = cln_rpc::model::requests::GetinfoRequest {};
    let res = cln_client.lock().await.call(Getinfo(req)).await.unwrap();

    let data = serde_json::to_string_pretty(&json!(res)).unwrap();
    format!("```json\n{}\n```", data)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("cln_info").description("Get cln node info")
}
