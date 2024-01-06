use std::str::FromStr;

use bitcoin::secp256k1::PublicKey;
use fedimint_client::ClientArc;
use fedimint_ln_client::LightningClientModule;
use serde_json::json;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use crate::commands::{discord_command_options_to_map, CommandOptionInfo};
use crate::utils::get_option_as::get_option_as;
use crate::utils::to_codeblock;

pub async fn run(options: &[CommandDataOption], fm_client: &ClientArc) -> String {
    let options_map = discord_command_options_to_map(options);
    let gateway_id = get_option_as::<String>(&options_map, "gateway_id").unwrap();
    let public_key = PublicKey::from_str(&gateway_id);
    let public_key = match public_key {
        Ok(public_key) => public_key,
        Err(e) => return format!("Error: {}", e),
    };
    let lightning_module = fm_client.get_first_module::<LightningClientModule>();
    let res = lightning_module.set_active_gateway(&public_key).await;
    if let Err(e) = res {
        return format!("Error: {}", e);
    }
    let gateway = lightning_module.select_active_gateway().await;
    let gateway = match gateway {
        Ok(gateway) => gateway,
        Err(e) => return format!("Error: {}", e),
    };
    let mut gateway_json = json!(&gateway);
    gateway_json["active"] = json!(true);

    to_codeblock(serde_json::to_string_pretty(&gateway_json).unwrap())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let options = vec![CommandOptionInfo {
        name: "gateway_id",
        description: "The id of the gateway to switch to",
        kind: CommandOptionType::String,
        required: true,
    }];

    command
        .name("fm_ln_list_gateways")
        .description("List the available lightning gateways");

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
