use std::sync::Arc;

use cln_rpc::primitives::{AmountOrAll, Feerate, Outpoint};
use cln_rpc::ClnRpc;
use cln_rpc::Request::Withdraw;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::CommandDataOption;
use serenity::model::prelude::command::CommandOptionType;
use tokio::sync::Mutex;

use super::format_json;
use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let destination: String = get_option_as(&options_map, "destination").unwrap();
    let amount: AmountOrAll = get_option_as(&options_map, "amount").unwrap();
    let feerate: Option<Feerate> = get_option_as(&options_map, "feerate");
    let minconf: Option<u16> = get_option_as(&options_map, "minconf");
    let utxos: Option<Vec<Outpoint>> = get_option_as(&options_map, "utxos");

    let req = cln_rpc::model::requests::WithdrawRequest {
        destination,
        satoshi: Some(amount),
        feerate,
        minconf,
        utxos,
    };

    match cln_client.lock().await.call(Withdraw(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "destination",
            description: "The destination address",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "amount",
            description: "The amount to be withdrawn",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "feerate",
            description: "The feerate",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "minconf",
            description: "The minimum number of confirmations",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "utxos",
            description: "The utxos to be used",
            kind: CommandOptionType::String,
            required: false,
        },
    ];

    command
        .name("cln_withdraw")
        .description("Withdraw funds from the internal wallet");

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
