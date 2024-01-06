use std::sync::Arc;

use cln_rpc::model::requests::DatastoreMode;
use cln_rpc::ClnRpc;
use cln_rpc::Request::Datastore;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::format_json;
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let key: Vec<String> = get_option_as(&options_map, "key").unwrap();
    let string: Option<String> = get_option_as(&options_map, "string");
    let hex: Option<String> = get_option_as(&options_map, "hex");
    let mode: Option<DatastoreMode> = get_option_as(&options_map, "mode");
    let generation: Option<u64> = get_option_as(&options_map, "generation");

    let req = cln_rpc::model::requests::DatastoreRequest {
        key,
        string,
        hex,
        mode,
        generation,
    };

    match cln_client.lock().await.call(Datastore(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "key",
            description: "The key hierarchy for the data",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "string",
            description: "The string data to store",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "hex",
            description: "The hex data to store",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "mode",
            description: "The mode for storing the data",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "generation",
            description: "The generation for atomic updates",
            kind: CommandOptionType::Integer,
            required: false,
        },
    ];

    command
        .name("cln_datastore")
        .description("Store data in the Core Lightning database");

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
