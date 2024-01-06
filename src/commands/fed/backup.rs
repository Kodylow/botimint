use fedimint_client::backup::Metadata;
use fedimint_client::ClientArc;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOption;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;
use crate::utils::to_codeblock;

pub async fn run(options: &[CommandDataOption], fm_client: &ClientArc) -> String {
    let options_map = discord_command_options_to_map(options);
    let metadata: String = get_option_as(&options_map, "metadata").unwrap();
    let metadata: Metadata = serde_json::from_str(&metadata).unwrap();
    let res = fm_client
        .backup_to_federation(metadata)
        .await
        .map_err(|e| e.to_string());
    to_codeblock(serde_json::to_string_pretty(&res).unwrap())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![CommandOptionInfo {
        name: "metadata",
        description: "The metadata to backup (as a stringified JSON object)",
        kind: CommandOptionType::String,
        required: true,
    }];
    command
        .name("fm_backup")
        .description("Get the federation id");

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
