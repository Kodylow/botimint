use std::sync::Arc;

use cln_rpc::primitives::PublicKey;
use cln_rpc::ClnRpc;
use cln_rpc::Request::CheckMessage;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::format_json;
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let message: String = get_option_as(&options_map, "message").unwrap();
    let zbase: String = get_option_as(&options_map, "zbase").unwrap();
    let pubkey: Option<PublicKey> = get_option_as(&options_map, "pubkey");

    let req = cln_rpc::model::requests::CheckmessageRequest {
        message,
        zbase,
        pubkey,
    };

    match cln_client.lock().await.call(CheckMessage(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "message",
            description: "The message to check",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "zbase",
            description: "The signature of the message",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "pubkey",
            description: "The public key of the node",
            kind: CommandOptionType::String,
            required: false,
        },
    ];

    command
        .name("cln_checkmessage")
        .description("Check if a signature is from a node");

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
