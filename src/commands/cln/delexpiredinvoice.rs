use std::sync::Arc;

use cln_rpc::ClnRpc;
use cln_rpc::Request::DelExpiredInvoice;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::option_utils::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let maxexpirytime: Option<u64> = get_option_as(&options_map, "maxexpirytime");

    let req = cln_rpc::model::requests::DelexpiredinvoiceRequest { maxexpirytime };

    match cln_client.lock().await.call(DelExpiredInvoice(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![CommandOptionInfo {
        name: "maxexpirytime",
        description: "The maximum expiry time for the invoices",
        kind: CommandOptionType::Integer,
        required: false,
    }];

    command
        .name("cln_delexpiredinvoice")
        .description("Remove expired invoices from the Core Lightning database");

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