use fedimint_client::Client;

pub struct Fedimint {
    pub client: Client,
}

impl Fedimint {
    pub async fn new() -> Result<Self> {
        let mut client_builder = fedimint_client::Client::builder();
        client_builder.with_module(WalletClientGen(None));
        client_builder.with_module(MintClientGen);
        client_builder.with_module(LightningClientGen);
        client_builder.with_database(MemDatabase::new());
        client_builder.with_primary_module(1);
        client_builder.with_invite_code(invite_code);
        let client_res = client_builder.build::<PlainRootSecretStrategy>().await;
    }
}
