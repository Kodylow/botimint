use std::sync::Arc;

use cln_rpc::ClnRpc;
use cln_rpc::Request::Stop;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;

pub async fn run(_options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let req = cln_rpc::model::requests::StopRequest {};
    let res = cln_client.lock().await.call(Stop(req)).await.unwrap();

    format_json(res)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("cln_stop")
        .description("Command to shutdown the Core Lightning node")
}
