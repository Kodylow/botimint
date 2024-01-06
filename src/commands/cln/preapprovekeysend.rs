use std::sync::Arc;

use cln_rpc::primitives::{Amount, PublicKey};
use cln_rpc::ClnRpc;
use cln_rpc::Request::PreApproveKeysend;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::format_json;
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let destination: Option<PublicKey> = get_option_as(&options_map, "destination");
    let payment_hash: Option<String> = get_option_as(&options_map, "payment_hash");
    let amount_msat: Option<Amount> = get_option_as(&options_map, "amount_msat");

    let req = cln_rpc::model::requests::PreapprovekeysendRequest {
        destination,
        payment_hash,
        amount_msat,
    };
    let res = cln_client
        .lock()
        .await
        .call(PreApproveKeysend(req))
        .await
        .unwrap();

    format_json(res)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "destination",
            description: "The public key of the destination node",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "payment_hash",
            description: "The unique identifier of a payment",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "amount_msat",
            description: "The amount to send in millisatoshi precision",
            kind: CommandOptionType::String,
            required: true,
        },
    ];

    command
        .name("cln_preapprovekeysend")
        .description("Ask the HSM to preapprove a keysend payment");

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
