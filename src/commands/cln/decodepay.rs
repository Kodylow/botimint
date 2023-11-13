use std::sync::Arc;

use cln_rpc::ClnRpc;
use cln_rpc::Request::DecodePay;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let bolt11: String = get_option_as(&options_map, "bolt11").unwrap();
    let description: Option<String> = get_option_as(&options_map, "description");

    let req = cln_rpc::model::requests::DecodepayRequest {
        bolt11,
        description,
    };
    let res = cln_client.lock().await.call(DecodePay(req)).await.unwrap();

    format_json(res)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "bolt11",
            description: "The bolt11 string",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "description",
            description: "The description of the purpose of the purchase",
            kind: CommandOptionType::String,
            required: false,
        },
    ];

    command
        .name("cln_decodepay")
        .description("Checks and parses a bolt11 string");

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
