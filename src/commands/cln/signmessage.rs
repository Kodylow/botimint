use std::sync::Arc;

use cln_rpc::ClnRpc;
use cln_rpc::Request::SignMessage;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use crate::commands::discord_command_options_to_map;
use crate::utils::format_json;
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let message: String = get_option_as(&options_map, "message").unwrap();

    let req = cln_rpc::model::requests::SignmessageRequest { message };
    let res = cln_client
        .lock()
        .await
        .call(SignMessage(req))
        .await
        .unwrap();

    format_json(res)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("cln_signmessage")
        .description("Create a digital signature of a message using this node's secret key")
        .create_option(|opt| {
            opt.name("message")
                .description("The message to sign")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
