use std::sync::Arc;

use cln_rpc::model::requests::FeeratesStyle;
use cln_rpc::ClnRpc;
use cln_rpc::Request::Feerates;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::format_json;
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let style: FeeratesStyle = get_option_as(&options_map, "style").unwrap();

    let req = cln_rpc::model::requests::FeeratesRequest { style };
    let res = cln_client.lock().await.call(Feerates(req)).await.unwrap();

    format_json(res)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![CommandOptionInfo {
        name: "style",
        description: "The style of feerate (either 'perkw' or 'perkb')",
        kind: CommandOptionType::String,
        required: true,
    }];

    command
        .name("cln_feerates")
        .description("Command for querying recommended onchain feerates");

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
