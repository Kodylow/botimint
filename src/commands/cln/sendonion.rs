use std::sync::Arc;

use cln_rpc::model::requests::SendonionFirst_hop;
use cln_rpc::primitives::{Amount, PublicKey, Secret, Sha256};
use cln_rpc::ClnRpc;
use cln_rpc::Request::SendOnion;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let onion: String = get_option_as(&options_map, "onion").unwrap();
    let first_hop: SendonionFirst_hop = get_option_as(&options_map, "first_hop").unwrap();
    let payment_hash: Sha256 = get_option_as(&options_map, "payment_hash").unwrap();
    let label: Option<String> = get_option_as(&options_map, "label");
    let shared_secrets: Option<Vec<Secret>> = get_option_as(&options_map, "shared_secrets");
    let partid: Option<u16> = get_option_as(&options_map, "partid");
    let bolt11: Option<String> = get_option_as(&options_map, "bolt11");
    let amount_msat: Option<Amount> = get_option_as(&options_map, "amount_msat");
    let destination: Option<PublicKey> = get_option_as(&options_map, "destination");
    let localinvreqid: Option<Sha256> = get_option_as(&options_map, "localinvreqid");
    let groupid: Option<u64> = get_option_as(&options_map, "groupid");

    let req = cln_rpc::model::requests::SendonionRequest {
        onion,
        first_hop,
        payment_hash,
        label,
        shared_secrets,
        partid,
        bolt11,
        amount_msat,
        destination,
        localinvreqid,
        groupid,
    };

    match cln_client.lock().await.call(SendOnion(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "onion",
            description: "The custom onion packet",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "first_hop",
            description: "The first hop to send the onion to",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "payment_hash",
            description: "The payment hash to use as a challenge",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "label",
            description: "A human readable reference for the payment",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "shared_secrets",
            description: "The shared secrets used when creating the onion",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "partid",
            description: "The partid value for parallel partial payments",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "bolt11",
            description: "The bolt11 parameter to be returned in results",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "amount_msat",
            description: "The amount_msat parameter to annotate the payment",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "destination",
            description: "The destination parameter to be returned in results",
            kind: CommandOptionType::String,
            required: false,
        },
    ];

    command
        .name("cln_sendonion")
        .description("Send a payment with a custom onion packet");

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
