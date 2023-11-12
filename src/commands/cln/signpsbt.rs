use std::sync::Arc;

use cln_rpc::model::requests::SignpsbtRequest;
use cln_rpc::ClnRpc;
use cln_rpc::Request::SignPsbt;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::CommandDataOption;
use serenity::model::prelude::command::CommandOptionType;
use tokio::sync::Mutex;

use super::format_json;
use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let psbt: String = get_option_as(&options_map, "psbt").unwrap();
    let signonly: Option<Vec<u32>> = get_option_as(&options_map, "signonly");

    let req = SignpsbtRequest { psbt, signonly };

    match cln_client.lock().await.call(SignPsbt(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "psbt",
            description: "The PSBT value",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "signonly",
            description: "An optional array of input numbers to sign",
            kind: CommandOptionType::Integer,
            required: false,
        },
    ];

    command.name("cln_signpsbt").description("Sign a PSBT");

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
