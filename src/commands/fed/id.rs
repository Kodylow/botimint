use fedimint_client::ClientArc;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub async fn run(_options: &[CommandDataOption], fm_client: &ClientArc) -> String {
    let federation_id = fm_client.federation_id();

    format!("Your federation id is: {}", federation_id)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("fm_federation_id")
        .description("Get the federation id")
}
