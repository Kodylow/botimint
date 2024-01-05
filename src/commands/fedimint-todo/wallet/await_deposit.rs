use crate::{error::AppError, state::AppState};
use anyhow::anyhow;
use axum::{extract::State, http::StatusCode, Json};
use fedimint_core::core::OperationId;
use fedimint_wallet_client::{DepositState, WalletClientModule};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct AwaitDepositRequest {
    pub operation_id: OperationId,
}

#[derive(Debug, Serialize)]
pub struct AwaitDepositResponse {
    pub status: DepositState,
}

async fn _await_deposit(
    state: AppState,
    req: AwaitDepositRequest,
) -> Result<AwaitDepositResponse, AppError> {
    let mut updates = state
        .fm
        .get_first_module::<WalletClientModule>()
        .subscribe_deposit_updates(req.operation_id)
        .await?
        .into_stream();

    while let Some(update) = updates.next().await {
        match update {
            DepositState::Confirmed(tx) => {
                return Ok(AwaitDepositResponse {
                    status: DepositState::Confirmed(tx),
                })
            }
            DepositState::Claimed(tx) => {
                return Ok(AwaitDepositResponse {
                    status: DepositState::Claimed(tx),
                })
            }
            DepositState::Failed(reason) => {
                return Err(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    anyhow!(reason),
                ))
            }
            _ => {}
        }
    }

    Err(AppError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        anyhow!("Unexpected end of stream"),
    ))
}

pub async fn handle_ws(v: Value, state: AppState) -> Result<Value, AppError> {
    let v = serde_json::from_value(v).unwrap();
    let await_deposit = _await_deposit(state, v).await?;
    let await_deposit_json = json!(await_deposit);
    Ok(await_deposit_json)
}

#[axum_macros::debug_handler]
pub async fn handle_rest(
    State(state): State<AppState>,
    Json(req): Json<AwaitDepositRequest>,
) -> Result<Json<AwaitDepositResponse>, AppError> {
    let await_deposit = _await_deposit(state, req).await?;
    Ok(Json(await_deposit))
}
