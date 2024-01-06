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
pub mod ln;
pub mod mint;
pub mod wallet;

pub enum FmCommand {
    Backup,
    Config,
    DiscoverVersion,
    Id,
    Info,
    ListOperations,
    Unknown,
}

impl From<&str> for FmCommand {
    fn from(s: &str) -> Self {
        match s {
            "fm_backup" => Self::Backup,
            "fm_config" => Self::Config,
            "fm_discover_version" => Self::DiscoverVersion,
            "fm_id" => Self::Id,
            "fm_info" => Self::Info,
            "fm_list_operations" => Self::ListOperations,
            _ => Self::Unknown,
        }
    }
}

pub async fn ready(ctx: &Context) {
    let commands = vec![
        backup::register,
        config::register,
        discover_version::register,
        id::register,
        info::register,
        list_operations::register,
    ];

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
        FmCommand::Config => config::run(&command_data.options, fm_client).await,
        FmCommand::DiscoverVersion => discover_version::run(&command_data.options, fm_client).await,
        FmCommand::Id => id::run(&command_data.options, fm_client).await,
        FmCommand::Info => info::run(&command_data.options, fm_client).await,
        FmCommand::ListOperations => list_operations::run(&command_data.options, fm_client).await,
        FmCommand::Unknown => format!("Unknown command: {}", command_name),
    }
}
