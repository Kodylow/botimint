use std::str::FromStr;

use fedimint_client::ClientArc;
use fedimint_core::core::OperationId;
use fedimint_ln_client::{LightningClientModule, LnReceiveState};
use futures::StreamExt;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOption;

use crate::commands::fed::ln::get_note_summary;
use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;
use crate::utils::to_codeblock;

pub async fn run(options: &[CommandDataOption], fm_client: &ClientArc) -> String {
    let options_map = discord_command_options_to_map(options);
    let operation_id = get_option_as::<String>(&options_map, "operation_id").unwrap();
    let operation_id = OperationId::from_str(&operation_id).unwrap();

    let lightning_module = &fm_client.get_first_module::<LightningClientModule>();
    let updates = lightning_module.subscribe_ln_receive(operation_id).await;
    let mut updates = match updates {
        Ok(updates) => updates.into_stream(),
        Err(e) => return format!("Error: {}", e),
    };
    while let Some(update) = updates.next().await {
        match update {
            LnReceiveState::Claimed => {
                let res = get_note_summary(&fm_client).await;
                match res {
                    Ok(res) => return to_codeblock(serde_json::to_string_pretty(&res).unwrap()),
                    Err(e) => return format!("Error: {}", e),
                }
            }
            LnReceiveState::Canceled { reason } => {
                return format!("Ln receive payment cancelled: {}", reason);
            }
            _ => {}
        }
    }

    format!("Error: end of stream for operation_id {}", operation_id)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![CommandOptionInfo {
        name: "operation_id",
        description: "The operation id of the ln invoice to await",
        kind: CommandOptionType::String,
        required: true,
    }];
    command
        .name("fm_ln_await_invoice")
        .description("Await an LN invoice");

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
