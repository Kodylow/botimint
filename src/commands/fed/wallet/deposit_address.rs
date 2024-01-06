use std::time::Duration;

use bitcoin::Address;
use fedimint_client::ClientArc;
use fedimint_core::core::OperationId;
use fedimint_core::time::now;
use fedimint_wallet_client::WalletClientModule;
use serde::Serialize;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOption;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;
use crate::utils::to_codeblock;

#[derive(Debug, Serialize)]
pub struct DepositAddressResponse {
    pub address: Address,
    pub operation_id: OperationId,
}

pub async fn run(options: &[CommandDataOption], fm_client: &ClientArc) -> String {
    let options_map = discord_command_options_to_map(options);
    let timeout: u64 = get_option_as(&options_map, "timeout").unwrap();
    let res = fm_client
        .get_first_module::<WalletClientModule>()
        .get_deposit_address(now() + Duration::from_secs(timeout), ())
        .await;

    match res {
        Ok((operation_id, address)) => {
            let res = DepositAddressResponse {
                address,
                operation_id,
            };
            to_codeblock(serde_json::to_string_pretty(&res).unwrap())
        }
        Err(e) => to_codeblock(serde_json::to_string_pretty(&format!("Error: {}", e)).unwrap()),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![CommandOptionInfo {
        name: "timeout",
        description: "The timeout for the pegin address in seconds",
        kind: CommandOptionType::Integer,
        required: true,
    }];

    command
        .name("fm_wallet_deposit_address")
        .description("Create a pegin deposit address");

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
