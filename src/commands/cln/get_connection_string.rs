use std::sync::Arc;

use cln_rpc::model::responses::GetinfoResponse;
use cln_rpc::Request::Getinfo;
use cln_rpc::{ClnRpc, Response};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

// TODO: fix this
pub async fn run(_options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let req = cln_rpc::model::requests::GetinfoRequest {};
    let res: GetinfoResponse = match cln_client.lock().await.call(Getinfo(req)).await.unwrap() {
        Response::Getinfo(res) => res,
        _ => unreachable!(),
    };

    // read my ip addr from bash command
    let my_ip_addr = std::process::Command::new("bash")
        .arg("-c")
        .arg("hostname -I | awk '{print $1}'")
        .output()
        .expect("failed to execute process")
        .stdout;

    format!(
        "My node's connection string is:\n`{:?}@{:?}:{:?}`",
        res.id,
        String::from_utf8(my_ip_addr).unwrap(),
        res.binding.unwrap()[0].port
    )
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("cln_get_connection_string")
        .description("Get this node's connection string")
}
