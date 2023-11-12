use std::sync::Arc;

use cln_rpc::ClnRpc;
use cln_rpc::Request::ListDatastore;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::option_utils::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let key: Option<Vec<String>> = get_option_as(&options_map, "key");

    let req = cln_rpc::model::requests::ListdatastoreRequest { key };

    match cln_client.lock().await.call(ListDatastore(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![CommandOptionInfo {
        name: "key",
        description: "The key hierarchy for the data",
        kind: CommandOptionType::String,
        required: false,
    }];

    command
        .name("cln_listdatastore")
        .description("Fetch data from the Core Lightning database");

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
