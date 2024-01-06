use std::sync::Arc;

use cln_rpc::model::requests::UtxopsbtRequest;
use cln_rpc::primitives::{Amount, Feerate, Outpoint};
use cln_rpc::ClnRpc;
use cln_rpc::Request::UtxoPsbt;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::CommandDataOption;
use serenity::model::prelude::command::CommandOptionType;
use tokio::sync::Mutex;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::format_json;
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let satoshi: Amount = get_option_as(&options_map, "satoshi").unwrap();
    let feerate: Feerate = get_option_as(&options_map, "feerate").unwrap();
    let startweight: u32 = get_option_as(&options_map, "startweight").unwrap();
    let utxos: Vec<Outpoint> = get_option_as(&options_map, "utxos").unwrap();
    let reserve: Option<u32> = get_option_as(&options_map, "reserve");
    let reservedok: Option<bool> = get_option_as(&options_map, "reservedok");
    let locktime: Option<u32> = get_option_as(&options_map, "locktime");
    let min_witness_weight: Option<u32> = get_option_as(&options_map, "min_witness_weight");
    let excess_as_change: Option<bool> = get_option_as(&options_map, "excess_as_change");
    let opening_anchor_channel: Option<bool> =
        get_option_as(&options_map, "opening_anchor_channel");

    let req = UtxopsbtRequest {
        satoshi,
        feerate,
        startweight,
        utxos,
        reserve,
        reservedok,
        locktime,
        min_witness_weight,
        excess_as_change,
        opening_anchor_channel,
    };

    match cln_client.lock().await.call(UtxoPsbt(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "satoshi",
            description: "The amount of satoshi",
            kind: CommandOptionType::Integer,
            required: true,
        },
        CommandOptionInfo {
            name: "feerate",
            description: "The feerate",
            kind: CommandOptionType::Integer,
            required: true,
        },
        CommandOptionInfo {
            name: "startweight",
            description: "The startweight",
            kind: CommandOptionType::Integer,
            required: true,
        },
        CommandOptionInfo {
            name: "utxos",
            description: "The UTXOs to use",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "reserve",
            description: "The reserve",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "reservedok",
            description: "Whether to reserve",
            kind: CommandOptionType::Boolean,
            required: false,
        },
        CommandOptionInfo {
            name: "locktime",
            description: "The locktime",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "min_witness_weight",
            description: "The min witness weight",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "excess_as_change",
            description: "Whether to use excess as change",
            kind: CommandOptionType::Boolean,
            required: false,
        },
        CommandOptionInfo {
            name: "opening_anchor_channel",
            description: "Whether to open an anchor channel",
            kind: CommandOptionType::Boolean,
            required: false,
        },
    ];

    command
        .name("cln_utxopsbt")
        .description("Populate PSBT inputs from given UTXOs");

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
