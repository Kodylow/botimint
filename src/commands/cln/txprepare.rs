use std::sync::Arc;

use cln_rpc::model::requests::TxprepareRequest;
use cln_rpc::primitives::{Feerate, Outpoint, OutputDesc};
use cln_rpc::ClnRpc;
use cln_rpc::Request::TxPrepare;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::CommandDataOption;
use serenity::model::prelude::command::CommandOptionType;
use tokio::sync::Mutex;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::format_json;
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let outputs: Vec<OutputDesc> = get_option_as(&options_map, "outputs").unwrap();
    let feerate: Option<Feerate> = get_option_as(&options_map, "feerate");
    let minconf: Option<u32> = get_option_as(&options_map, "minconf");
    let utxos: Option<Vec<Outpoint>> = get_option_as(&options_map, "utxos");

    let req = TxprepareRequest {
        outputs,
        feerate,
        minconf,
        utxos,
    };

    match cln_client.lock().await.call(TxPrepare(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "outputs",
            description: "The outputs to prepare the transaction for",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "feerate",
            description: "The feerate to use for the transaction",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "minconf",
            description: "The minimum number of confirmations that used outputs should have",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "utxos",
            description: "The utxos to be used to fund the transaction",
            kind: CommandOptionType::String,
            required: false,
        },
    ];

    command
        .name("cln_txprepare")
        .description("Prepare to withdraw funds from the internal wallet");

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
