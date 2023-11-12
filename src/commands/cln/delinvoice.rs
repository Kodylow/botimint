use std::sync::Arc;

use cln_rpc::model::requests::DelinvoiceStatus;
use cln_rpc::ClnRpc;
use cln_rpc::Request::DelInvoice;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let label: String = get_option_as(&options_map, "label").unwrap();
    let status: DelinvoiceStatus = get_option_as(&options_map, "status").unwrap();
    let desconly: Option<bool> = get_option_as(&options_map, "desconly");

    let req = cln_rpc::model::requests::DelinvoiceRequest {
        label,
        status,
        desconly,
    };

    match cln_client.lock().await.call(DelInvoice(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "label",
            description: "The label of the invoice",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "status",
            description: "The status of the invoice",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "desconly",
            description: "Whether to only remove the description",
            kind: CommandOptionType::Boolean,
            required: false,
        },
    ];

    command
        .name("cln_delinvoice")
        .description("Remove an invoice or its description from the Core Lightning database");

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
