use std::str::FromStr;

use fedimint_client::ClientArc;
use fedimint_core::Amount;
use fedimint_mint_client::{MintClientModule, OOBNotes};
use futures::StreamExt;
use serde::Serialize;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOption;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;
use crate::utils::to_codeblock;

#[derive(Debug, Serialize)]
pub struct ReissueResponse {
    pub amount_msat: Amount,
}

pub async fn run(options: &[CommandDataOption], fm_client: &ClientArc) -> String {
    let options_map = discord_command_options_to_map(options);
    let notes = get_option_as::<String>(&options_map, "notes").unwrap();
    let notes: OOBNotes = OOBNotes::from_str(&notes).unwrap();
    let amount_msat = notes.total_amount();
    let mint = fm_client.get_first_module::<MintClientModule>();

    let operation_id = mint.reissue_external_notes(notes, ()).await;
    let operation_id = match operation_id {
        Ok(operation_id) => operation_id,
        Err(e) => return format!("Error: {}", e),
    };

    let mut updates = mint
        .subscribe_reissue_external_notes(operation_id)
        .await
        .unwrap()
        .into_stream();

    while let Some(update) = updates.next().await {
        if let fedimint_mint_client::ReissueExternalNotesState::Failed(e) = update {
            return format!("Error: {}", e);
        }
    }

    let res = ReissueResponse { amount_msat };

    to_codeblock(serde_json::to_string_pretty(&res).unwrap())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![CommandOptionInfo {
        name: "notes",
        description: "The notes to reissue",
        kind: CommandOptionType::String,
        required: true,
    }];
    command
        .name("fm_mint_reissue")
        .description("Reissue ecash notes");

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
