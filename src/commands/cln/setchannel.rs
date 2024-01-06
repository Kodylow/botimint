use std::sync::Arc;

use cln_rpc::primitives::Amount;
use cln_rpc::ClnRpc;
use cln_rpc::Request::SetChannel;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::format_json;
use crate::utils::get_option_as::get_option_as;

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let id: String = get_option_as(&options_map, "id").unwrap();
    let feebase: Option<Amount> = get_option_as(&options_map, "feebase");
    let feeppm: Option<u32> = get_option_as(&options_map, "feeppm");
    let htlcmin: Option<Amount> = get_option_as(&options_map, "htlcmin");
    let htlcmax: Option<Amount> = get_option_as(&options_map, "htlcmax");
    let enforcedelay: Option<u32> = get_option_as(&options_map, "enforcedelay");
    let ignorefeelimits: Option<bool> = get_option_as(&options_map, "ignorfeelimits");

    let req = cln_rpc::model::requests::SetchannelRequest {
        id,
        feebase,
        feeppm,
        htlcmin,
        htlcmax,
        enforcedelay,
        ignorefeelimits,
    };
    let res = cln_client.lock().await.call(SetChannel(req)).await.unwrap();

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
            name: "feebase",
            description: "Base fee to add to any routed payment",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "feeppm",
            description: "Fee added proportionally per-millionths to any routed payment volume",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "htlcmin",
            description: "Limits how small an HTLC will be forwarded",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "htlcmax",
            description: "Limits how large an HTLC will be forwarded",
            kind: CommandOptionType::String,
            required: false,
        },
        CommandOptionInfo {
            name: "enforcedelay",
            description: "Delay before enforcing the new fees/htlc max",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "ignorfeelimits",
            description: "Ignore the limits set by other peers",
            kind: CommandOptionType::Boolean,
            required: false,
        },
    ];

    command.name("cln_setchannel").description(
        "Set channel specific routing fees, and htlc_minimum_msat or htlc_maximum_msat",
    );

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
