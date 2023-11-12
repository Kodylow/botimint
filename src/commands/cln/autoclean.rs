use std::sync::Arc;

use cln_rpc::ClnRpc;
use cln_rpc::Request::AutoCleanInvoice;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let expired_by: Option<u64> = get_option_as(&options_map, "expired_by");
    let cycle_seconds: Option<u64> = get_option_as(&options_map, "cycle_seconds");

    let req = cln_rpc::model::requests::AutocleaninvoiceRequest {
        expired_by,
        cycle_seconds,
    };

    match cln_client.lock().await.call(AutoCleanInvoice(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "expired_by",
            description: "The time in seconds that the invoice should be expired by",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "cycle_seconds",
            description: "The time in seconds between each autoclean",
            kind: CommandOptionType::Integer,
            required: false,
        },
    ];

    command
        .name("cln_autoclean")
        .description("Automatically clean up expired invoices");

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
