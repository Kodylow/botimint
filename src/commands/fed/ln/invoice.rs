use fedimint_client::ClientArc;
use fedimint_core::core::OperationId;
use fedimint_core::Amount;
use fedimint_ln_client::LightningClientModule;
use serde::Serialize;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOption;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;
use crate::utils::to_codeblock;

#[derive(Debug, Serialize)]
pub struct LnInvoiceResponse {
    pub operation_id: OperationId,
    pub invoice: String,
}

pub async fn run(options: &[CommandDataOption], fm_client: &ClientArc) -> String {
    let options_map = discord_command_options_to_map(options);
    let amount_msat = get_option_as::<u64>(&options_map, "amount_msat").unwrap();
    let amount_msat = Amount::from_msats(amount_msat);
    let description = get_option_as::<String>(&options_map, "description").unwrap();
    let expiry_time = get_option_as::<u64>(&options_map, "expiry_time").unwrap_or(3600);
    let lightning_module = fm_client.get_first_module::<LightningClientModule>();
    let res = lightning_module.select_active_gateway().await;
    if let Err(e) = res {
        return format!("Error: {}", e);
    }

    let res = lightning_module
        .create_bolt11_invoice(amount_msat, description, Some(expiry_time), ())
        .await;

    let res = match res {
        Ok(res) => LnInvoiceResponse {
            operation_id: res.0,
            invoice: res.1.to_string(),
        },
        Err(e) => return format!("Error: {}", e),
    };

    to_codeblock(serde_json::to_string_pretty(&res).unwrap())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "amount_msat",
            description: "The amount in msat to receive",
            kind: CommandOptionType::Integer,
            required: true,
        },
        CommandOptionInfo {
            name: "description",
            description: "The description for the ln invoice",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "expiry_time",
            description: "The expiry time for the ln invoice",
            kind: CommandOptionType::Integer,
            required: false,
        },
    ];
    command
        .name("fm_ln_invoice")
        .description("Create an LN invoice");

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
