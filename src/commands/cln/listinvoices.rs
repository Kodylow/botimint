use std::sync::Arc;

use cln_rpc::model::requests::ListinvoicesIndex;
use cln_rpc::ClnRpc;
use cln_rpc::Request::ListInvoices;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::format_json;
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let label: Option<String> = get_option_as(&options_map, "label");
    let invstring: Option<String> = get_option_as(&options_map, "invstring");
    let payment_hash: Option<String> = get_option_as(&options_map, "payment_hash");
    let offer_id: Option<String> = get_option_as(&options_map, "offer_id");
    let index: Option<ListinvoicesIndex> = get_option_as(&options_map, "index");
    let start: Option<u64> = get_option_as(&options_map, "start");
    let limit: Option<u32> = get_option_as(&options_map, "limit");

    let req = cln_rpc::model::requests::ListinvoicesRequest {
        label,
        invstring,
        payment_hash,
        offer_id,
        index,
        start,
        limit,
    };

    match cln_client.lock().await.call(ListInvoices(req)).await {
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
            required: false,
        },
        CommandOptionInfo {
            name: "invstring",
            description: "The string representing the invoice",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "payment_hash",
            description: "The payment hash of the invoice",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "offer_id",
            description: "The local offer id this invoice was issued for",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "index",
            description: "The index for listing invoices",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "start",
            description: "The start point for listing invoices",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "limit",
            description: "The limit for listing invoices",
            kind: CommandOptionType::Integer,
            required: false,
        },
    ];

    command
        .name("cln_listinvoices")
        .description("Query the status of invoices");

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
