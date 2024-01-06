use fedimint_client::ClientArc;
use fedimint_ln_client::LightningClientModule;
use serde_json::json;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use crate::utils::to_codeblock;

pub async fn run(_options: &[CommandDataOption], fm_client: &ClientArc) -> String {
    let lightning_module = fm_client.get_first_module::<LightningClientModule>();
    let gateways = lightning_module.fetch_registered_gateways().await;
    let gateways = match gateways {
        Ok(gateways) => gateways,
        Err(e) => return format!("Error: {}", e),
    };
    if gateways.is_empty() {
        return "No gateways registered".to_string();
    }

    let mut gateways_json = json!(&gateways);
    let active_gateway = lightning_module.select_active_gateway().await;
    let active_gateway = match active_gateway {
        Ok(active_gateway) => active_gateway,
        Err(e) => return format!("Error: {}", e),
    };

    gateways_json
        .as_array_mut()
        .expect("gateways_json is not an array")
        .iter_mut()
        .for_each(|gateway| {
            if gateway["node_pub_key"] == json!(active_gateway.node_pub_key) {
                gateway["active"] = json!(true);
            } else {
                gateway["active"] = json!(false);
            }
        });

    to_codeblock(serde_json::to_string_pretty(&gateways_json).unwrap())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("fm_ln_list_gateways")
        .description("List the available lightning gateways")
}
