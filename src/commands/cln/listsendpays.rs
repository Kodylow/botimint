use std::sync::Arc;

use cln_rpc::model::requests::ListsendpaysStatus;
use cln_rpc::primitives::Sha256;
use cln_rpc::ClnRpc;
use cln_rpc::Request::ListSendPays;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let bolt11: Option<String> = get_option_as(&options_map, "bolt11");
    let payment_hash: Option<Sha256> = get_option_as(&options_map, "payment_hash");
    let status: Option<ListsendpaysStatus> = get_option_as(&options_map, "status");

    let req = cln_rpc::model::requests::ListsendpaysRequest {
        bolt11,
        payment_hash,
        status,
    };

    match cln_client.lock().await.call(ListSendPays(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "bolt11",
            description: "The bolt11 parameter to be returned in results",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "payment_hash",
            description: "The payment hash to use as a challenge",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "status",
            description: "The status of the payment",
            kind: CommandOptionType::String,
            required: false,
        },
    ];

    command
        .name("cln_listsendpays")
        .description("List the status of all sendpay commands");

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
