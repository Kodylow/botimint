use fedimint_client::ClientArc;
use serenity::model::prelude::application_command::CommandData;
use serenity::prelude::Context;

use crate::utils::discord_utils::create_and_log_command;

pub mod id;
pub mod info;

pub enum FmCommand {
    Id,
    Info,
    Unknown,
}

impl From<&str> for FmCommand {
    fn from(s: &str) -> Self {
        match s {
            "fm_id" => Self::Id,
            "fm_info" => Self::Info,
            _ => Self::Unknown,
        }
    }
}

pub async fn ready(ctx: &Context) {
    let commands = vec![id::register, info::register];

    for command in commands {
        create_and_log_command(&ctx.http, command).await;
    }
}

pub async fn handle_run(
    command_name: &str,
    command_data: &CommandData,
    fm_client: &ClientArc,
) -> String {
    match FmCommand::from(command_name) {
        FmCommand::Id => id::run(&command_data.options, fm_client).await,
        FmCommand::Info => info::run(&command_data.options, fm_client).await,
        FmCommand::Unknown => format!("Unknown command: {}", command_name),
    }
}
