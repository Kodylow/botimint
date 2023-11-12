use std::sync::Arc;

use cln_rpc::primitives::Sha256;
use cln_rpc::ClnRpc;
use cln_rpc::Request::WaitSendPay;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::CommandDataOption;
use serenity::model::prelude::command::CommandOptionType;
use tokio::sync::Mutex;

use super::format_json;
use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let payment_hash: Sha256 = get_option_as(&options_map, "payment_hash").unwrap();
    let timeout: Option<u32> = get_option_as(&options_map, "timeout");
    let partid: Option<u64> = get_option_as(&options_map, "partid");
    let groupid: Option<u64> = get_option_as(&options_map, "groupid");

    let req = cln_rpc::model::requests::WaitsendpayRequest {
        payment_hash,
        timeout,
        partid,
        groupid,
    };

    match cln_client.lock().await.call(WaitSendPay(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "payment_hash",
            description: "The payment hash to use as a challenge",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "timeout",
            description: "The timeout in seconds",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "partid",
            description: "The partid value for parallel partial payments",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "groupid",
            description: "The groupid value for parallel partial payments",
            kind: CommandOptionType::Integer,
            required: false,
        },
    ];

    command
        .name("cln_waitsendpay")
        .description("Wait for the status of an outgoing payment");

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
