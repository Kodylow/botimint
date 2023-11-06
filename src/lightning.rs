use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use cln_rpc::ClnRpc;
use tokio::sync::{MappedMutexGuard, Mutex, MutexGuard};

pub struct Cln(Arc<Mutex<ClnRpc>>);

impl Cln {
    pub async fn new(path: &PathBuf) -> Result<Self> {
        let client = ClnRpc::new(path).await;
        Ok(Self(Arc::new(Mutex::new(client?))))
    }

    pub async fn client_lock(&self) -> anyhow::Result<MappedMutexGuard<'_, ClnRpc>> {
        let guard = self.0.lock().await;
        Ok(MutexGuard::map(guard, |client| client))
    }
}
