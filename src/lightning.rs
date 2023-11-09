use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use cln_rpc::ClnRpc;
use tokio::sync::Mutex;

pub async fn new_cln(path: &PathBuf) -> Result<Arc<Mutex<ClnRpc>>> {
    let client = ClnRpc::new(path).await;
    Ok(Arc::new(Mutex::new(client?)))
}
