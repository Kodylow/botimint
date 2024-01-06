use std::sync::Arc;

use cln_rpc::primitives::{Amount, AmountOrAll, Feerate, Outpoint, PublicKey};
use cln_rpc::ClnRpc;
use cln_rpc::Request::FundChannel;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::format_json;
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let id: PublicKey = get_option_as(&options_map, "id").unwrap();
    let amount: AmountOrAll = get_option_as(&options_map, "amount").unwrap();
    let feerate: Feerate = get_option_as(&options_map, "feerate").unwrap_or(Feerate::PerKb(1000));
    let announce: Option<bool> = get_option_as(&options_map, "announce");
    let minconf: Option<u32> = get_option_as(&options_map, "minconf");
    let push_msat: Option<Amount> = get_option_as(&options_map, "push_msat");
    let close_to: Option<String> = get_option_as(&options_map, "close_to");
    let request_amt: Option<Amount> = get_option_as(&options_map, "request_amt");
    let compact_lease: Option<String> = get_option_as(&options_map, "compact_lease");
    let utxos: Option<Vec<Outpoint>> = get_option_as(&options_map, "utxos");
    let mindepth: Option<u32> = get_option_as(&options_map, "mindepth");
    let reserve: Option<Amount> = get_option_as(&options_map, "reserve");

    let req = cln_rpc::model::requests::FundchannelRequest {
        id,
        amount,
        feerate: Some(feerate),
        announce,
        minconf,
        push_msat,
        close_to,
        request_amt,
        compact_lease,
        utxos,
        mindepth,
        reserve,
    };

    match cln_client.lock().await.call(FundChannel(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "id",
            description: "The public key of the peer",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "amount",
            description: "The amount to fund the channel with",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "feerate",
            description: "Fee rate in satoshi per kiloweight",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "announce",
            description: "Whether to announce this channel",
            kind: CommandOptionType::Boolean,
            required: false,
        },
        CommandOptionInfo {
            name: "minconf",
            description: "Minimum number of confirmations",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "push_msat",
            description: "Amount in millisatoshis to push to the peer",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "close_to",
            description: "Address to close channel to",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "request_amt",
            description: "Amount requested by the other side of the channel",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "compact_lease",
            description: "Compact lease of the channel",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "utxos",
            description: "The utxos to use for the channel",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "mindepth",
            description: "Minimum depth for the funding transaction",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "reserve",
            description: "Reserve amount for the channel",
            kind: CommandOptionType::String,
            required: false,
        },
    ];

    command
        .name("cln_fundchannel")
        .description("Open a payment channel with a peer");

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
