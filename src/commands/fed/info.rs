use std::collections::BTreeMap;

use fedimint_client::ClientArc;
use fedimint_core::config::FederationId;
use fedimint_core::{Amount, TieredSummary};
use fedimint_mint_client::MintClientModule;
use fedimint_wallet_client::WalletClientModule;
use serde::Serialize;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::CommandDataOption;

use crate::utils::to_codeblock;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct InfoResponse {
    pub federation_id: FederationId,
    pub network: String,
    pub meta: BTreeMap<String, String>,
    pub total_amount_msat: Amount,
    pub total_num_notes: usize,
    pub denominations_msat: TieredSummary,
}

pub async fn run(_options: &[CommandDataOption], fm_client: &ClientArc) -> String {
    let mint_client = fm_client.get_first_module::<MintClientModule>();
    let wallet_client = fm_client.get_first_module::<WalletClientModule>();
    let summary = mint_client
        .get_wallet_summary(
            &mut fm_client
                .db()
                .begin_transaction_nc()
                .await
                .to_ref_with_prefix_module_id(1),
        )
        .await;
    let response = InfoResponse {
        federation_id: fm_client.federation_id(),
        network: wallet_client.get_network().to_string(),
        meta: fm_client.get_config().global.meta.clone(),
        total_amount_msat: summary.total_amount(),
        total_num_notes: summary.count_items(),
        denominations_msat: summary,
    };

    to_codeblock(serde_json::to_string_pretty(&response).unwrap())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("fm_federation_id")
        .description("Get the federation id")
}
