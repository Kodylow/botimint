use std::sync::Arc;

use cln_rpc::model::requests::FundpsbtRequest;
use cln_rpc::primitives::{AmountOrAll, Feerate};
use cln_rpc::ClnRpc;
use cln_rpc::Request::FundPsbt;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let satoshi: AmountOrAll = get_option_as(&options_map, "satoshi").unwrap();
    let feerate: Feerate = get_option_as(&options_map, "feerate").unwrap();
    let startweight: u32 = get_option_as(&options_map, "startweight").unwrap();
    let minconf: Option<u32> = get_option_as(&options_map, "minconf");
    let reserve: Option<u32> = get_option_as(&options_map, "reserve");
    let locktime: Option<u32> = get_option_as(&options_map, "locktime");
    let min_witness_weight: Option<u32> = get_option_as(&options_map, "min_witness_weight");
    let excess_as_change: Option<bool> = get_option_as(&options_map, "excess_as_change");
    let nonwrapped: Option<bool> = get_option_as(&options_map, "nonwrapped");
    let opening_anchor_channel: Option<bool> =
        get_option_as(&options_map, "opening_anchor_channel");

    let req = FundpsbtRequest {
        satoshi,
        feerate,
        startweight,
        minconf,
        reserve,
        locktime,
        min_witness_weight,
        excess_as_change,
        nonwrapped,
        opening_anchor_channel,
    };

    match cln_client.lock().await.call(FundPsbt(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "satoshi",
            description: "The minimum satoshi value of the output(s) needed",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "feerate",
            description: "The feerate for the transaction",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "startweight",
            description: "The weight of the transaction before fundpsbt has added any inputs",
            kind: CommandOptionType::Integer,
            required: true,
        },
    ];

    command
        .name("cln_fundpsbt")
        .description("Populate PSBT inputs from the wallet");

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
