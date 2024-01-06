use std::str::FromStr;

use bitcoin::Amount;
use fedimint_client::ClientArc;
use fedimint_wallet_client::{WalletClientModule, WithdrawState};
use futures::StreamExt;
use serde::Serialize;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOption;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;
use crate::utils::to_codeblock;

#[derive(Debug, Serialize)]
pub struct WithdrawResponse {
    pub txid: String,
    pub fees_sat: u64,
}

pub async fn run(options: &[CommandDataOption], fm_client: &ClientArc) -> String {
    let options_map = discord_command_options_to_map(options);
    let address: String = get_option_as(&options_map, "address").unwrap();
    let address = bitcoin::Address::from_str(&address).unwrap();
    let amount_msat: u64 = get_option_as(&options_map, "amount_msat").unwrap();
    let amount = Amount::from_sat(amount_msat);
    let wallet_module = fm_client.get_first_module::<WalletClientModule>();
    let fees = wallet_module
        .get_withdraw_fees(address.clone(), amount)
        .await
        .map_err(|e| e.to_string());
    let fees = match fees {
        Ok(fees) => fees,
        Err(e) => {
            return to_codeblock(serde_json::to_string_pretty(&format!("Error: {}", e)).unwrap())
        }
    };
    let absolute_fees = fees.amount();

    let operation_id = wallet_module
        .withdraw(address, amount, fees, ())
        .await
        .map_err(|e| e.to_string());
    let operation_id = match operation_id {
        Ok(operation_id) => operation_id,
        Err(e) => {
            return to_codeblock(serde_json::to_string_pretty(&format!("Error: {}", e)).unwrap())
        }
    };

    let updates = wallet_module.subscribe_withdraw_updates(operation_id).await;
    let mut updates = match updates {
        Ok(stream) => stream.into_stream(),
        Err(e) => {
            return to_codeblock(serde_json::to_string_pretty(&format!("Error: {}", e)).unwrap())
        }
    };

    while let Some(update) = updates.next().await {
        match update {
            WithdrawState::Succeeded(txid) => {
                let res = WithdrawResponse {
                    txid: txid.to_string(),
                    fees_sat: absolute_fees.to_sat(),
                };

                return to_codeblock(serde_json::to_string_pretty(&res).unwrap());
            }
            WithdrawState::Failed(e) => {
                return to_codeblock(
                    serde_json::to_string_pretty(&format!("Error: {}", e)).unwrap(),
                )
            }
            _ => continue,
        };
    }

    to_codeblock(serde_json::to_string_pretty("Error: Unexpected end of stream").unwrap())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "address",
            description: "The address to withdraw to",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "amount_msat",
            description: "The amount to withdraw (in msat)",
            kind: CommandOptionType::Integer,
            required: true,
        },
    ];

    command
        .name("fm_wallet_withdraw")
        .description("Withdraw via pegout to an onchain address");

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
