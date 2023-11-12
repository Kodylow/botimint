use std::sync::Arc;

use cln_rpc::ClnRpc;
use cln_rpc::Request::WaitAnyInvoice;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::CommandDataOption;
use serenity::model::prelude::command::CommandOptionType;
use tokio::sync::Mutex;

use super::format_json;
use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let lastpay_index: Option<u64> = get_option_as(&options_map, "lastpay_index");
    let timeout: Option<u64> = get_option_as(&options_map, "timeout");

    let req = cln_rpc::model::requests::WaitanyinvoiceRequest {
        lastpay_index,
        timeout,
    };

    match cln_client.lock().await.call(WaitAnyInvoice(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "lastpay_index",
            description: "The last pay index",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "timeout",
            description: "The timeout in seconds",
            kind: CommandOptionType::Integer,
            required: false,
        },
    ];

    command
        .name("cln_waitanyinvoice")
        .description("Wait until an invoice is paid");

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
