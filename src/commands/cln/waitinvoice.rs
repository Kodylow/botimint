use std::sync::Arc;

use cln_rpc::ClnRpc;
use cln_rpc::Request::WaitInvoice;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::CommandDataOption;
use serenity::model::prelude::command::CommandOptionType;
use tokio::sync::Mutex;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::format_json;
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let label: String = get_option_as(&options_map, "label").unwrap();

    let req = cln_rpc::model::requests::WaitinvoiceRequest { label };

    match cln_client.lock().await.call(WaitInvoice(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![CommandOptionInfo {
        name: "label",
        description: "The label of the invoice",
        kind: CommandOptionType::String,
        required: true,
    }];

    command
        .name("cln_waitinvoice")
        .description("Wait until a specific invoice is paid");

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
