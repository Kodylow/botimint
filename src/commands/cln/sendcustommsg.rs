use std::sync::Arc;

use cln_rpc::primitives::PublicKey;
use cln_rpc::ClnRpc;
use cln_rpc::Request::SendCustomMsg;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::format_json;
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let node_id: PublicKey = get_option_as(&options_map, "node_id").unwrap();
    let msg: String = get_option_as(&options_map, "msg").unwrap();

    let req = cln_rpc::model::requests::SendcustommsgRequest { node_id, msg };
    let res = cln_client
        .lock()
        .await
        .call(SendCustomMsg(req))
        .await
        .unwrap();

    format_json(res)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "node_id",
            description: "The public key of the peer",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "msg",
            description: "The custom message to send",
            kind: CommandOptionType::String,
            required: true,
        },
    ];

    command
        .name("cln_sendcustommsg")
        .description("Low-level interface to send protocol messages to peers");

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
