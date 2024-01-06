use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use cln_rpc::primitives::PublicKey;
use cln_rpc::ClnRpc;
use cln_rpc::Request::Connect;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::Mutex;

use crate::commands::discord_command_options_to_map;
use crate::utils::format_json;
use crate::utils::get_option_as::get_option_as;

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

    // fn to_string(&self) -> String {
    //     format!("{}@{}:{}", self.id, self.host, self.port)
    // }
}

pub async fn run(options: &[CommandDataOption], cln_client: &Arc<Mutex<ClnRpc>>) -> String {
    let options_map = discord_command_options_to_map(options);
    let connection_string: Option<String> = get_option_as(&options_map, "connection_string");

    match ConnectionString::from_string(&connection_string.unwrap()) {
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
            format!("Error: {}", e)
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
