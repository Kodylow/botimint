use std::sync::Arc;

use cln_rpc::primitives::Amount;
use cln_rpc::ClnRpc;
use cln_rpc::Request::Pay;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::format_json;
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let bolt11: String = get_option_as(&options_map, "bolt11").unwrap();
    let amount_msat: Option<Amount> = get_option_as(&options_map, "amount_msat");
    let label: Option<String> = get_option_as(&options_map, "label");
    let riskfactor: Option<f64> = get_option_as(&options_map, "riskfactor");
    let maxfeepercent: Option<f64> = get_option_as(&options_map, "maxfeepercent");
    let retry_for: Option<u16> = get_option_as(&options_map, "retry_for");
    let maxdelay: Option<u16> = get_option_as(&options_map, "maxdelay");
    let exemptfee: Option<Amount> = get_option_as(&options_map, "exemptfee");
    let localinvreqid: Option<String> = get_option_as(&options_map, "localinvreqid");
    let exclude: Option<Vec<String>> = get_option_as(&options_map, "exclude");
    let maxfee: Option<Amount> = get_option_as(&options_map, "maxfee");
    let description: Option<String> = get_option_as(&options_map, "description");

    let req = cln_rpc::model::requests::PayRequest {
        bolt11,
        amount_msat,
        label,
        riskfactor,
        maxfeepercent,
        retry_for,
        maxdelay,
        exemptfee,
        localinvreqid,
        exclude,
        maxfee,
        description,
    };

    match cln_client.lock().await.call(Pay(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "bolt11",
            description: "The BOLT11 invoice to pay",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "amount_msat",
            description: "The amount to pay in millisatoshis",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "label",
            description: "A label for the payment",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "riskfactor",
            description: "Risk factor for route calculation",
            kind: CommandOptionType::Number,
            required: false,
        },
        CommandOptionInfo {
            name: "maxfeepercent",
            description: "Maximum fee as a percentage of the amount",
            kind: CommandOptionType::Number,
            required: false,
        },
        CommandOptionInfo {
            name: "retry_for",
            description: "Time in seconds to keep retrying the payment",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "maxdelay",
            description: "Maximum delay for the payment in blocks",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "exemptfee",
            description: "Fee exemption amount in millisatoshis",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "localinvreqid",
            description: "Local invoice request id",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "exclude",
            description: "Exclude a channel from the route",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "maxfee",
            description: "Maximum fee in millisatoshis",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "description",
            description: "Description of the payment",
            kind: CommandOptionType::String,
            required: false,
        },
    ];

    command
        .name("cln_pay")
        .description("Send a payment to a BOLT11 invoice");

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
