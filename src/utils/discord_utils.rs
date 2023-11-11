use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Result;
use cln_rpc::primitives::{Amount, Feerate, Outpoint, PublicKey, Sha256};
use serde_json::Value;
use serenity::model::prelude::command::Command;
use tracing::info;

pub async fn create_and_log_command<F>(http: &serenity::http::Http, command_register: F)
where
    F: FnOnce(
            &mut serenity::builder::CreateApplicationCommand,
        ) -> &mut serenity::builder::CreateApplicationCommand
        + Send
        + Sync,
{
    let command = Command::create_global_application_command(http, command_register)
        .await
        .unwrap();
    info!("Created Slash Command: {:#?}", command.name);
}
