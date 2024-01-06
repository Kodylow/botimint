use std::sync::Arc;

use cln_rpc::primitives::AmountOrAny;
use cln_rpc::ClnRpc;
use cln_rpc::Request::Invoice;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::format_json;
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let amount_msat: AmountOrAny = get_option_as(&options_map, "amount_msat").unwrap();
    let label: String = get_option_as(&options_map, "label").unwrap();
    let description: String = get_option_as(&options_map, "description").unwrap();
    let expiry: Option<u64> = get_option_as(&options_map, "expiry");
    let fallbacks: Option<Vec<String>> = get_option_as(&options_map, "fallbacks");
    let preimage: Option<String> = get_option_as(&options_map, "preimage");
    let cltv: Option<u32> = get_option_as(&options_map, "cltv");
    let deschashonly: Option<bool> = get_option_as(&options_map, "deschashonly");

    let req = cln_rpc::model::requests::InvoiceRequest {
        amount_msat,
        label,
        description,
        expiry,
        fallbacks,
        preimage,
        cltv,
        deschashonly,
    };

    match cln_client.lock().await.call(Invoice(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "amount_msat",
            description: "The amount in milli-satoshi",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "label",
            description: "The label of the invoice",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "description",
            description: "The description of the invoice",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "expiry",
            description: "The expiry time of the invoice",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "fallbacks",
            description: "The fallback addresses",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "preimage",
            description: "The preimage of the invoice",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "cltv",
            description: "The cltv value",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "deschashonly",
            description: "Whether to only use the description hash",
            kind: CommandOptionType::Boolean,
            required: false,
        },
    ];

    command
        .name("cln_invoice")
        .description("Create an invoice for accepting payments");

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
