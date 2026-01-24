use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post, put},
};
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{
    application::vault::VaultUseCase,
    controller::app_state::AppState,
    model::{
        app_error::AppResult,
        jwt::Claims,
        response::SuccessResponse,
        vault::{UpdateVaultRequest, VaultRequest, Vaults},
    },
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_vault))
        .route("/", put(update_vault))
        .route("/{id}", delete(delete_vault))
        .route("/all", get(get_all_vaults))
}

#[instrument(name = "create_vault", skip(payload, vault_use_case, claims))]
pub async fn create_vault(
    claims: Claims,
    State(vault_use_case): State<Arc<VaultUseCase>>,
    Json(payload): Json<VaultRequest>,
) -> AppResult<Json<SuccessResponse<()>>> {
    vault_use_case.create_vault(claims.sub, payload).await?;

    Ok(Json(SuccessResponse {
        data: None,
        message: "Successfully create new vault".to_string(),
    }))
}

#[instrument(name = "get_all_vaults", skip(vault_use_case, claims))]
pub async fn get_all_vaults(
    claims: Claims,
    State(vault_use_case): State<Arc<VaultUseCase>>,
) -> AppResult<Json<SuccessResponse<Vec<Vaults>>>> {
    info!("get_all_vaults");
    let vaults = vault_use_case.get_all_vaults(claims.sub).await?;

    info!("Successfully get all vaults");
    Ok(Json(SuccessResponse {
        data: Some(vaults),
        message: "Successfully get all vaults".to_string(),
    }))
}

#[instrument(name = "update_vault", skip(payload, vault_use_case, claims))]
pub async fn update_vault(
    claims: Claims,
    State(vault_use_case): State<Arc<VaultUseCase>>,
    Json(payload): Json<UpdateVaultRequest>,
) -> AppResult<Json<SuccessResponse<()>>> {
    vault_use_case.update_vault(claims.sub, payload).await?;

    Ok(Json(SuccessResponse {
        data: None,
        message: "Successfully update vault".to_string(),
    }))
}

#[instrument(name = "delete_vault", skip(vault_use_case, claims, id))]
pub async fn delete_vault(
    claims: Claims,
    State(vault_use_case): State<Arc<VaultUseCase>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<SuccessResponse<()>>> {
    vault_use_case.delete_vault(claims.sub, id).await?;

    Ok(Json(SuccessResponse {
        data: None,
        message: "Successfully delete vault".to_string(),
    }))
}
