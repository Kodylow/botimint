use serenity::model::prelude::application_command::CommandData;
use serenity::prelude::Context;

use crate::utils::discord_utils::create_and_log_command;

pub mod ping;
pub mod user_id;

pub enum CustomCommand {
    Id,
    Ping,
    Unknown,
}

impl From<&str> for CustomCommand {
    fn from(s: &str) -> Self {
        match s {
            "id" => Self::Id,
            "ping" => Self::Ping,
            _ => Self::Unknown,
        }
    }
}

pub async fn ready(ctx: &Context) {
    let commands = vec![user_id::register, ping::register];

    for command in commands {
        create_and_log_command(&ctx.http, command).await;
    }
}

pub async fn handle_run(command_name: &str, command_data: &CommandData) -> String {
    match CustomCommand::from(command_name) {
        CustomCommand::Id => user_id::run(&command_data.options),
        CustomCommand::Ping => ping::run(&command_data.options),
        CustomCommand::Unknown => format!("Unknown command: {}", command_name),
    }
}
