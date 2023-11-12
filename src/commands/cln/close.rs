use std::sync::Arc;

use cln_rpc::primitives::{Feerate, Outpoint};
use cln_rpc::ClnRpc;
use cln_rpc::Request::Close;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let id: String = get_option_as(&options_map, "id").unwrap();
    let unilateraltimeout: Option<u32> = get_option_as(&options_map, "unilateraltimeout");
    let destination: Option<String> = get_option_as(&options_map, "destination");
    let fee_negotiation_step: Option<String> = get_option_as(&options_map, "fee_negotiation_step");
    let wrong_funding: Option<Outpoint> = get_option_as(&options_map, "wrong_funding");
    let force_lease_closed: Option<bool> = get_option_as(&options_map, "force_lease_closed");
    let feerange: Option<Vec<Feerate>> = get_option_as(&options_map, "feerange");

    let req = cln_rpc::model::requests::CloseRequest {
        id,
        unilateraltimeout,
        destination,
        fee_negotiation_step,
        wrong_funding,
        force_lease_closed,
        feerange,
    };

    match cln_client.lock().await.call(Close(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "id",
            description: "The id of the channel or peer",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "unilateraltimeout",
            description: "The unilateral timeout in seconds",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "destination",
            description: "The destination address for the to-local output",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "fee_negotiation_step",
            description: "The step for fee negotiation",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "wrong_funding",
            description: "The wrong funding transaction id and output number",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "force_lease_closed",
            description: "Force close of leased funds",
            kind: CommandOptionType::Boolean,
            required: false,
        },
        CommandOptionInfo {
            name: "feerange",
            description: "The minimum and maximum feerates to offer",
            kind: CommandOptionType::Integer,
            required: false,
        },
    ];

    command
        .name("cln_close")
        .description("Close a channel with a direct peer");

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
