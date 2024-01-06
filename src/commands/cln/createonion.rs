use std::sync::Arc;

use cln_rpc::model::requests::CreateonionHops;
use cln_rpc::primitives::Secret;
use cln_rpc::ClnRpc;
use cln_rpc::Request::CreateOnion;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::format_json;
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let hops: Vec<CreateonionHops> = get_option_as(&options_map, "hops").unwrap();
    let assocdata: String = get_option_as(&options_map, "assocdata").unwrap();
    let session_key: Option<Secret> = get_option_as(&options_map, "session_key");
    let onion_size: Option<u16> = get_option_as(&options_map, "onion_size");

    let req = cln_rpc::model::requests::CreateonionRequest {
        hops,
        assocdata,
        session_key,
        onion_size,
    };

    match cln_client.lock().await.call(CreateOnion(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "hops",
            description: "The hops for the onion",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "assocdata",
            description: "The associated data for the onion",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "session_key",
            description: "The session key for the onion",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "onion_size",
            description: "The size of the onion",
            kind: CommandOptionType::Integer,
            required: false,
        },
    ];

    command
        .name("cln_createonion")
        .description("Create a custom onion");

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
