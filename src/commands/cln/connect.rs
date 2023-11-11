use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use cln_rpc::primitives::PublicKey;
use cln_rpc::ClnRpc;
use cln_rpc::Request::Connect;
use serde_json::json;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use super::format_json;
use crate::utils::get_option_as_string;

struct ConnectionString {
    id: String,
    host: String,
    port: u16,
}

impl ConnectionString {
    fn from_string(s: &str) -> Result<ConnectionString> {
        let mut parts = s.split('@');
        let id = parts.next().unwrap().to_string();
        assert!(PublicKey::from_str(&id).is_ok());
        let mut parts = parts.next().unwrap().split(':');
        let host = parts.next().unwrap().to_string();
        let port = parts.next().unwrap().parse::<u16>()?;

        Ok(ConnectionString { id, host, port })
    }

    fn to_string(&self) -> String {
        format!("{}@{}:{}", self.id, self.host, self.port)
    }
}

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let opt_str = get_option_as_string(options[0].clone()).unwrap_or_default();

    match ConnectionString::from_string(&opt_str) {
        Ok(cs) => {
            let req = cln_rpc::model::requests::ConnectRequest {
                id: cs.id,
                host: Some(cs.host),
                port: Some(cs.port),
            };
            let res = cln_client.lock().await.call(Connect(req)).await.unwrap();
            format_json(res)
        }
        Err(e) => {
            return format!("Error: {}", e);
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("cln_connect")
        .description("Connect to a peer")
        .create_option(|opt| {
            opt.name("connection_string")
                .description("The connection string of the peer")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
