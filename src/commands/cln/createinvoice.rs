use std::sync::Arc;

use cln_rpc::ClnRpc;
use cln_rpc::Request::CreateInvoice;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use crate::utils::format_json;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = crate::commands::discord_command_options_to_map(options);
    let invstring: Option<String> =
        crate::utils::get_option_as::get_option_as(&options_map, "invstring");
    // random label if not provided
    let label: Option<String> = crate::utils::get_option_as::get_option_as(&options_map, "label")
        .or(Some(format!("botimint-{}", uuid::Uuid::new_v4())));
    let preimage: Option<String> =
        crate::utils::get_option_as::get_option_as(&options_map, "preimage");

    let req = cln_rpc::model::requests::CreateinvoiceRequest {
        invstring: invstring.unwrap(),
        label: label.unwrap(),
        preimage: preimage.unwrap(),
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
                .required(false)
        })
        .create_option(|opt| {
            opt.name("label")
                .description("A unique label for the invoice")
                .kind(CommandOptionType::String)
                .required(false)
        })
        .create_option(|opt| {
            opt.name("preimage")
                .description("The preimage for the invoice")
                .kind(CommandOptionType::String)
                .required(false)
        })
}
