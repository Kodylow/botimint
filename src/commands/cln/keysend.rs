use std::sync::Arc;

use cln_rpc::primitives::{Amount, PublicKey, RoutehintList, TlvStream};
use cln_rpc::ClnRpc;
use cln_rpc::Request::KeySend;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let destination: PublicKey = get_option_as(&options_map, "destination").unwrap();
    let amount_msat: Amount = get_option_as(&options_map, "amount_msat").unwrap();
    let label: Option<String> = get_option_as(&options_map, "label");
    let maxfeepercent: Option<f64> = get_option_as(&options_map, "maxfeepercent");
    let retry_for: Option<u32> = get_option_as(&options_map, "retry_for");
    let exemptfee: Option<Amount> = get_option_as(&options_map, "exemptfee");
    let maxdelay: Option<u32> = get_option_as(&options_map, "maxdelay");
    let routehints: Option<RoutehintList> = get_option_as(&options_map, "routehints");
    let extratlvs: Option<TlvStream> = get_option_as(&options_map, "extratlvs");

    let req = cln_rpc::model::requests::KeysendRequest {
        destination,
        amount_msat,
        label,
        maxfeepercent,
        retry_for,
        exemptfee,
        maxdelay,
        routehints,
        extratlvs,
    };

    match cln_client.lock().await.call(KeySend(req)).await {
        Ok(res) => format_json(res),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "destination",
            description: "The node ID of the node that the payment should go to",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "amount_msat",
            description: "The amount to send in millisatoshi precision",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "label",
            description: "A label to attach to the payment",
            kind: CommandOptionType::String,
            required: false,
        },
    ];

    command
        .name("cln_keysend")
        .description("Send funds to a node without an invoice");

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
