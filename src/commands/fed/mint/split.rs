use std::collections::BTreeMap;
use std::str::FromStr;

use fedimint_client::ClientArc;
use fedimint_core::{Amount, TieredMulti};
use fedimint_mint_client::OOBNotes;
use serde::Serialize;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOption;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;
use crate::utils::to_codeblock;

#[derive(Debug, Serialize)]
pub struct SplitResponse {
    pub notes: BTreeMap<Amount, OOBNotes>,
}

pub async fn run(options: &[CommandDataOption], _fm_client: &ClientArc) -> String {
    let options_map = discord_command_options_to_map(options);
    let notes = get_option_as::<String>(&options_map, "notes").unwrap();
    let notes: OOBNotes = OOBNotes::from_str(&notes).unwrap();
    let federation = notes.federation_id_prefix();
    let notes = notes
        .notes()
        .iter()
        .map(|(amount, notes)| {
            let notes = notes
                .iter()
                .map(|note| {
                    OOBNotes::new(
                        federation,
                        TieredMulti::new(vec![(*amount, vec![*note])].into_iter().collect()),
                    )
                })
                .collect::<Vec<_>>();
            (*amount, notes[0].clone()) // clone the amount and return a single
                                        // OOBNotes
        })
        .collect::<BTreeMap<_, _>>();

    let res = SplitResponse { notes };

    to_codeblock(serde_json::to_string_pretty(&res).unwrap())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![CommandOptionInfo {
        name: "notes",
        description: "The notes to split",
        kind: CommandOptionType::String,
        required: true,
    }];

    command
        .name("fm_mint_split")
        .description("Split ecash notes");

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
