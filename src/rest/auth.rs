use crate::{
    dto::{
        SignInRequest,
        AuthTokens,
        SignUpRequest
    },
    rest::error::Error, state::AppState,
};
use anyhow::Result;
use axum::{Json, extract::State, http::StatusCode};
use validator::Validate;

#[tracing::instrument(skip(state, req))]
pub async fn sign_up(
    State(state): State<AppState>,
    Json(req): Json<SignUpRequest>,
) -> Result<(StatusCode, Json<AuthTokens>), Error> {
    req.validate()?;

    let tokens = state
        .dependencies
        .auth_service()
        .await
        .sign_up(req.into())
        .await?;
    Ok((StatusCode::CREATED, Json(tokens)))
}

#[tracing::instrument(skip(state, req))]
pub async fn sign_in(
    State(state): State<AppState>,
    Json(req): Json<SignInRequest>,
) -> Result<(StatusCode, Json<AuthTokens>), Error> {
    req.validate()?;

    let tokens = state
        .dependencies
        .auth_service()
        .await
        .sign_in(req.into())
        .await?;
    Ok((StatusCode::OK, Json(tokens)))
}
