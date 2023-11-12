use std::sync::Arc;

use cln_rpc::primitives::{PublicKey, ShortChannelId};
use cln_rpc::ClnRpc;
use cln_rpc::Request::ListChannels;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let short_channel_id: Option<ShortChannelId> = get_option_as(&options_map, "short_channel_id");
    let source: Option<PublicKey> = get_option_as(&options_map, "source");
    let destination: Option<PublicKey> = get_option_as(&options_map, "destination");

    let req = cln_rpc::model::requests::ListchannelsRequest {
        short_channel_id,
        source,
        destination,
    };

    match cln_client.lock().await.call(ListChannels(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "short_channel_id",
            description: "The short channel id",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "source",
            description: "The source node id",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "destination",
            description: "The destination node id",
            kind: CommandOptionType::String,
            required: false,
        },
    ];

    command
        .name("cln_listchannels")
        .description("Query active lightning channels in the network");

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
