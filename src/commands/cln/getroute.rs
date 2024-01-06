use std::sync::Arc;

use cln_rpc::primitives::{Amount, PublicKey};
use cln_rpc::ClnRpc;
use cln_rpc::Request::GetRoute;
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
    let amount_msat: Amount = get_option_as(&options_map, "amount_msat").unwrap();
    let riskfactor: u64 = get_option_as(&options_map, "riskfactor").unwrap();
    let cltv: Option<u32> = get_option_as(&options_map, "cltv");
    let fromid: Option<PublicKey> = get_option_as(&options_map, "fromid");
    let fuzzpercent: Option<u32> = get_option_as(&options_map, "fuzzpercent");
    let exclude: Option<Vec<String>> = get_option_as(&options_map, "exclude");
    let maxhops: Option<u32> = get_option_as(&options_map, "maxhops");

    let req = cln_rpc::model::requests::GetrouteRequest {
        id,
        amount_msat,
        riskfactor,
        cltv,
        fromid,
        fuzzpercent,
        exclude,
        maxhops,
    };
    let res = cln_client.lock().await.call(GetRoute(req)).await.unwrap();

    format_json(res)
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
            name: "amount_msat",
            description: "The amount in millisatoshi",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "riskfactor",
            description: "The risk factor for the route",
            kind: CommandOptionType::Number,
            required: true,
        },
        // Add other options here...
    ];

    command
        .name("cln_getroute")
        .description("Get the best route for a payment");

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
