use fedimint_client::ClientArc;
use fedimint_core::Amount;
use fedimint_ln_client::{LightningClientModule, OutgoingLightningPayment};
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOption;
use tracing::info;

use super::{get_invoice, wait_for_ln_payment};
use crate::commands::fed::ln::LnPayRequest;
use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;
use crate::utils::to_codeblock;

pub async fn run(options: &[CommandDataOption], fm_client: &ClientArc) -> String {
    let options_map = discord_command_options_to_map(options);
    let payment_info = get_option_as::<String>(&options_map, "payment_info").unwrap();
    let amount_msat = get_option_as::<u64>(&options_map, "amount_msat").unwrap();
    let amount_msat = Amount::from_msats(amount_msat);
    let finish_in_background =
        get_option_as::<bool>(&options_map, "finish_in_background").unwrap_or(false);
    let lnurl_comment =
        get_option_as::<String>(&options_map, "lnurl_comment").unwrap_or("".to_string());
    let req = LnPayRequest {
        payment_info,
        amount_msat: Some(amount_msat),
        finish_in_background,
        lnurl_comment: Some(lnurl_comment),
    };

    let bolt11 = get_invoice(&req).await;
    let bolt11 = match bolt11 {
        Ok(bolt11) => bolt11,
        Err(e) => return format!("Error: {}", e),
    };
    let lightning_module = fm_client.get_first_module::<LightningClientModule>();
    let res = lightning_module.select_active_gateway().await;
    if let Err(e) = res {
        return format!("Error: {}", e);
    }

    let res = lightning_module.pay_bolt11_invoice(bolt11, ()).await;
    let res = match res {
        Ok(res) => res,
        Err(e) => return format!("Error: {}", e),
    };
    let OutgoingLightningPayment {
        payment_type,
        contract_id,
        fee,
    } = res;
    let operation_id = payment_type.operation_id();
    info!("Gateway fee: {fee}, payment operation id: {operation_id}");
    if req.finish_in_background {
        let res =
            wait_for_ln_payment(&fm_client, payment_type, contract_id.to_string(), true).await;
        match res {
        Ok(Some(res)) => {
            let pretty_res = serde_json::to_string_pretty(&res).unwrap();
            return format!("Payment will finish in background, use await-ln-pay to get the result\n{}", to_codeblock(pretty_res));
        }
        Ok(None) => return format!("Payment will finish in background, use await-ln-pay to get the result\nError: end of stream for operation_id {}", operation_id),
        Err(e) => return format!("Payment will finish in background, use await-ln-pay to get the result\nError: {}", e),
    }
    } else {
        let res =
            wait_for_ln_payment(&fm_client, payment_type, contract_id.to_string(), false).await;
        match res {
            Ok(Some(res)) => {
                let res = to_codeblock(serde_json::to_string_pretty(&res).unwrap());
                res
            }
            Ok(None) => format!("Error: end of stream for operation_id {}", operation_id),
            Err(e) => format!("Error: {}", e),
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![
        CommandOptionInfo {
            name: "payment_info",
            description: "The invoice to pay",
            kind: CommandOptionType::String,
            required: true,
        },
        CommandOptionInfo {
            name: "finish_in_background",
            description: "Whether to finish the payment in the background",
            kind: CommandOptionType::Boolean,
            required: true,
        },
        CommandOptionInfo {
            name: "amount_msat",
            description: "The amount to pay in millisatoshis",
            kind: CommandOptionType::Integer,
            required: false,
        },
        CommandOptionInfo {
            name: "lnurl_comment",
            description: "The comment to use for the lnurl",
            kind: CommandOptionType::String,
            required: false,
        },
    ];
    command.name("fm_ln_pay").description("Pay an LN invoice");

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
