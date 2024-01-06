use std::str::FromStr;

use fedimint_client::ClientArc;
use fedimint_core::Amount;
use fedimint_mint_client::{MintClientModule, OOBNotes};
use serde::Serialize;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOption;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;
use crate::utils::to_codeblock;

#[derive(Debug, Serialize)]
pub struct ValidateResponse {
    pub amount_msat: Amount,
}

pub async fn run(options: &[CommandDataOption], fm_client: &ClientArc) -> String {
    let options_map = discord_command_options_to_map(options);
    let notes = get_option_as::<String>(&options_map, "notes").unwrap();
    let notes = OOBNotes::from_str(&notes).unwrap();
    let amount_msat = fm_client
        .get_first_module::<MintClientModule>()
        .validate_notes(notes)
        .await;

    match amount_msat {
        Ok(amount_msat) => {
            let res = ValidateResponse { amount_msat };
            to_codeblock(serde_json::to_string_pretty(&res).unwrap())
        }
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![CommandOptionInfo {
        name: "notes",
        description: "The notes to validate",
        kind: CommandOptionType::String,
        required: true,
    }];

    command
        .name("fm_mint_validate")
        .description("Validate ecash notes");

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
