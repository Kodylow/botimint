use fedimint_client::ClientArc;
use serenity::model::prelude::application_command::CommandData;
use serenity::prelude::Context;

use crate::utils::discord_utils::create_and_log_command;

pub mod backup;
pub mod config;
pub mod discover_version;
pub mod id;
pub mod info;
pub mod list_operations;

pub enum FmCommand {
    Backup,
    Id,
    Info,
    Unknown,
}

impl From<&str> for FmCommand {
    fn from(s: &str) -> Self {
        match s {
            "fm_backup" => Self::Backup,
            "fm_id" => Self::Id,
            "fm_info" => Self::Info,
            _ => Self::Unknown,
        }
    }
}

pub async fn ready(ctx: &Context) {
    let commands = vec![backup::register, id::register, info::register];

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
        FmCommand::Backup => backup::run(&command_data.options, fm_client).await,
        FmCommand::Id => id::run(&command_data.options, fm_client).await,
        FmCommand::Info => info::run(&command_data.options, fm_client).await,
        FmCommand::Unknown => format!("Unknown command: {}", command_name),
    }
}
