use std::time::Duration;

use fedimint_client::ClientArc;
use fedimint_core::core::OperationId;
use fedimint_core::Amount;
use fedimint_mint_client::{MintClientModule, OOBNotes, SelectNotesWithAtleastAmount};
use serde::Serialize;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOption;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;
use crate::utils::to_codeblock;

#[derive(Debug, Serialize)]
pub struct SpendResponse {
    pub operation: OperationId,
    pub notes: OOBNotes,
}

pub async fn run(options: &[CommandDataOption], fm_client: &ClientArc) -> String {
    let options_map = discord_command_options_to_map(options);
    let amount_msat = get_option_as::<u64>(&options_map, "amount_msat").unwrap();
    let amount_msat = Amount::from_msats(amount_msat);
    let allow_overpay = get_option_as::<bool>(&options_map, "allow_overpay").unwrap_or(false);
    let timeout = get_option_as::<u64>(&options_map, "timeout").unwrap_or(60);

    let mint_module = fm_client.get_first_module::<MintClientModule>();
    let timeout = Duration::from_secs(timeout);
    let res = mint_module
        .spend_notes_with_selector(&SelectNotesWithAtleastAmount, amount_msat, timeout, ())
        .await;
    let (operation_id, notes) = match res {
        Ok(res) => res,
        Err(e) => return format!("Error: {}", e),
    };

    let overspend_amount = notes.total_amount() - amount_msat;
    if overspend_amount != Amount::ZERO {
        if !allow_overpay {
            return format!(
                "Error: The amount you want to spend is {} msat less than the amount of the notes you selected. \
                If you want to spend the notes anyway, use the `allow_overpay` option.",
                overspend_amount
            );
        }
    }

    let res = SpendResponse {
        operation: operation_id,
        notes,
    };

    to_codeblock(serde_json::to_string_pretty(&res).unwrap())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "amount_msat",
            description: "The amount to spend",
            kind: CommandOptionType::Integer,
            required: true,
        },
        CommandOptionInfo {
            name: "allow_overpay",
            description: "Allow overpaying",
            kind: CommandOptionType::Boolean,
            required: false,
        },
        CommandOptionInfo {
            name: "timeout",
            description: "Timeout in seconds",
            kind: CommandOptionType::Integer,
            required: false,
        },
    ];
    command
        .name("fm_mint_spend")
        .description("Spend ecash notes");

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
