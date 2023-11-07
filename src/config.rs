use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use fedimint_client::derivable_secret::DerivableSecret;
use fedimint_client::secret::{PlainRootSecretStrategy, RootSecretStrategy};
use fedimint_core::api::InviteCode;

pub struct Config {
    pub guild_id: String,
    pub discord_client_token: String,
    pub cln_rpc_path: PathBuf,
    pub invite_code: InviteCode,
    pub root_secret: DerivableSecret,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();
        let guild_id = env::var("GUILD_ID")?;
        let discord_client_token = env::var("DISCORD_CLIENT_TOKEN")?;
        let cln_rpc_path = PathBuf::from(env::var("CLN_RPC_PATH")?);
        let invite_code = match InviteCode::from_str(&env::var("FEDERATION_INVITE_CODE")?) {
            Ok(invite_code) => invite_code,
            Err(e) => panic!("Invalid invite code: {}", e),
        };

        // Read the secret from the environment
        let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

        let root_secret = create_root_secret(secret);

        Ok(Self {
            guild_id,
            discord_client_token,
            cln_rpc_path,
            invite_code,
            root_secret,
        })
    }
}

fn create_root_secret(secret: String) -> DerivableSecret {
    // Convert the secret to bytes
    let secret_bytes = secret.as_bytes();

    // Ensure the secret is 64 bytes long
    assert_eq!(secret_bytes.len(), 64, "SECRET_KEY must be 64 bytes long");

    // Convert the bytes to a fixed-size array
    let mut secret_array = [0; 64];
    secret_array.copy_from_slice(secret_bytes);

    // Convert the secret to a DerivableSecret
    let root_secret = PlainRootSecretStrategy::to_root_secret(&secret_array);
    root_secret
}
