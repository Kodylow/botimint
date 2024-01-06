use std::str::FromStr;

use fedimint_client::ClientArc;
use fedimint_core::core::OperationId;
use fedimint_ln_client::{LightningClientModule, PayType};
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOption;

use super::wait_for_ln_payment;
use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;
use crate::utils::to_codeblock;

pub async fn run(options: &[CommandDataOption], fm_client: &ClientArc) -> String {
    let options_map = discord_command_options_to_map(options);
    let operation_id = get_option_as::<String>(&options_map, "operation_id").unwrap();
    let operation_id = OperationId::from_str(&operation_id).unwrap();

    let lightning_module = fm_client.get_first_module::<LightningClientModule>();
    let ln_pay_details = lightning_module.get_ln_pay_details_for(operation_id).await;
    let ln_pay_details = match ln_pay_details {
        Ok(ln_pay_details) => ln_pay_details,
        Err(e) => return format!("Error: {}", e),
    };
    let payment_type = if ln_pay_details.is_internal_payment {
        PayType::Internal(operation_id)
    } else {
        PayType::Lightning(operation_id)
    };
    let res = wait_for_ln_payment(
        &fm_client,
        payment_type,
        ln_pay_details.contract_id.to_string(),
        false,
    )
    .await;
    match res {
        Ok(Some(res)) => {
            let res = to_codeblock(serde_json::to_string_pretty(&res).unwrap());
            res
        }
        Ok(None) => format!("Error: end of stream for operation_id {}", operation_id),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![CommandOptionInfo {
        name: "operation_id",
        description: "The operation id of the ln invoice to await",
        kind: CommandOptionType::String,
        required: true,
    }];
    command
        .name("fm_ln_await_pay")
        .description("Await an LN payment success or failure");

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
