use std::str::FromStr;

use fedimint_client::ClientArc;
use fedimint_core::core::OperationId;
use fedimint_wallet_client::{DepositState, WalletClientModule};
use futures::StreamExt;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOption;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;
use crate::utils::to_codeblock;

pub async fn run(options: &[CommandDataOption], fm_client: &ClientArc) -> String {
    let options_map = discord_command_options_to_map(options);
    let operation_id: String = get_option_as(&options_map, "operation_id").unwrap();
    let operation_id: OperationId = OperationId::from_str(&operation_id).unwrap();
    let updates_result = fm_client
        .get_first_module::<WalletClientModule>()
        .subscribe_deposit_updates(operation_id)
        .await;

    let mut updates = match updates_result {
        Ok(stream) => stream.into_stream(),
        Err(e) => {
            return to_codeblock(serde_json::to_string_pretty(&format!("Error: {}", e)).unwrap())
        }
    };

    while let Some(update) = updates.next().await {
        match update {
            DepositState::Confirmed(tx) => {
                return to_codeblock(serde_json::to_string_pretty(&tx).unwrap())
            }
            DepositState::Claimed(tx) => {
                return to_codeblock(serde_json::to_string_pretty(&tx).unwrap())
            }
            DepositState::Failed(reason) => {
                return to_codeblock(serde_json::to_string_pretty(&reason).unwrap())
            }
            _ => {}
        }
    }

    to_codeblock(serde_json::to_string_pretty("Error: Unexpected end of stream").unwrap())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![CommandOptionInfo {
        name: "operation_id",
        description: "The deposit operation id to await",
        kind: CommandOptionType::String,
        required: true,
    }];
    command
        .name("fm_wallet_await_deposit")
        .description("Await a peg in deposit by operation id");

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
