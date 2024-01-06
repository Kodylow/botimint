use std::sync::Arc;

use cln_rpc::ClnRpc;
use cln_rpc::Request::ListHtlcs;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::format_json;
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let id: Option<String> = get_option_as(&options_map, "id");

    let req = cln_rpc::model::requests::ListhtlcsRequest { id };
    let res = cln_client.lock().await.call(ListHtlcs(req)).await.unwrap();

    format_json(res)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![CommandOptionInfo {
        name: "id",
        description: "The public key of the peer",
        kind: CommandOptionType::String,
        required: false,
    }];

    command.name("cln_listhtlcs").description("Get all HTLCs");

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
