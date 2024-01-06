use fedimint_client::ClientArc;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::CommandDataOption;

use crate::utils::to_codeblock;

pub async fn run(_options: &[CommandDataOption], fm_client: &ClientArc) -> String {
    let res = fm_client
        .discover_common_api_version()
        .await
        .map_err(|e| e.to_string());
    to_codeblock(serde_json::to_string_pretty(&res).unwrap())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("fm_discover_version")
        .description("Get the common federation api version");

    command
}
