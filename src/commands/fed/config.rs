use fedimint_client::ClientArc;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::CommandDataOption;

use crate::utils::to_codeblock;

pub async fn run(_options: &[CommandDataOption], fm_client: &ClientArc) -> String {
    let res = fm_client.get_config_json();
    to_codeblock(serde_json::to_string_pretty(&res).unwrap())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("fm_config")
        .description("Get the federation config");

    command
}
