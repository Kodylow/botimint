use std::sync::Arc;

use cln_rpc::ClnRpc;
use cln_rpc::Request::CreateInvoice;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::utils::option_utils::get_option_as_string;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let invstring = get_option_as_string(options[0].clone()).unwrap_or_default();
    let label = get_option_as_string(options[1].clone()).unwrap_or_default();
    let preimage = get_option_as_string(options[2].clone()).unwrap_or_default();

    let req = cln_rpc::model::requests::CreateinvoiceRequest {
        invstring,
        label,
        preimage,
    };
    match cln_client.lock().await.call(CreateInvoice(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("cln_createinvoice")
        .description("Create a new invoice")
        .create_option(|opt| {
            opt.name("invstring")
                .description("The invoice string in bolt11 form")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|opt| {
            opt.name("label")
                .description("A unique label for the invoice")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|opt| {
            opt.name("preimage")
                .description("The preimage for the invoice")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
