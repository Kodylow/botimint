use std::sync::Arc;

use cln_rpc::model::requests::SendpayRoute;
use cln_rpc::primitives::{Amount, Secret, Sha256};
use cln_rpc::ClnRpc;
use cln_rpc::Request::SendPay;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::CommandDataOption;
use serenity::model::prelude::command::CommandOptionType;
use tokio::sync::Mutex;

use super::format_json;
use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let route: Vec<SendpayRoute> = get_option_as(&options_map, "route").unwrap();
    let payment_hash: Sha256 = get_option_as(&options_map, "payment_hash").unwrap();
    let label: Option<String> = get_option_as(&options_map, "label");
    let amount_msat: Option<Amount> = get_option_as(&options_map, "amount_msat");
    let bolt11: Option<String> = get_option_as(&options_map, "bolt11");
    let payment_secret: Option<Secret> = get_option_as(&options_map, "payment_secret");
    let partid: Option<u16> = get_option_as(&options_map, "partid");
    let localinvreqid: Option<String> = get_option_as(&options_map, "localinvreqid");
    let groupid: Option<u64> = get_option_as(&options_map, "groupid");

    let req = cln_rpc::model::requests::SendpayRequest {
        route,
        payment_hash,
        label,
        amount_msat,
        bolt11,
        payment_secret,
        partid,
        localinvreqid,
        groupid,
    };

    match cln_client.lock().await.call(SendPay(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "route",
            description: "The route for the payment",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "payment_hash",
            description: "The payment hash",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "label",
            description: "The label for the payment",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "amount_msat",
            description: "The amount in millisatoshis",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "bolt11",
            description: "The bolt11 invoice",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "payment_secret",
            description: "The payment secret",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "partid",
            description: "The partid",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "localinvreqid",
            description: "The local invoice request id",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "groupid",
            description: "The group id",
            kind: CommandOptionType::Integer,
            required: false,
        },
    ];

    command.name("cln_sendpay").description("Send a payment");

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
