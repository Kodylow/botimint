use std::sync::Arc;

use cln_rpc::primitives::PublicKey;
use cln_rpc::ClnRpc;
use cln_rpc::Request::Ping;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let id: PublicKey = get_option_as(&options_map, "id").unwrap();
    let len: Option<u16> = get_option_as(&options_map, "len");
    let pongbytes: Option<u16> = get_option_as(&options_map, "pongbytes");

    let req = cln_rpc::model::requests::PingRequest { id, len, pongbytes };

    match cln_client.lock().await.call(Ping(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "id",
            description: "The node id",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "len",
            description: "The length of the ping",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "pongbytes",
            description: "The length of the reply",
            kind: CommandOptionType::Integer,
            required: false,
        },
    ];

    command
        .name("cln_ping")
        .description("Check if a node is up");

    for opt_info in options {
        command.create_option(|opt| {
            opt.name(opt_info.name)
                .description(opt_info.description)
                .kind(opt_info.kind)
                .required(opt_info.required)
        });
    }

    command
}
