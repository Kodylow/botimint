use std::env;
use std::path::PathBuf;

pub struct Config {
    pub guild_id: String,
    pub discord_client_token: String,
    pub cln_rpc_path: PathBuf,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();
        let guild_id = env::var("GUILD_ID")?;
        let discord_client_token = env::var("DISCORD_CLIENT_TOKEN")?;
        let cln_rpc_path = PathBuf::from(env::var("CLN_RPC_PATH")?);

        Ok(Self {
            guild_id,
            discord_client_token,
            cln_rpc_path,
        })
    }
}
