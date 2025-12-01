use crate::{
    dependency::AppContainer, domain::auth::AuthTokens, dto::auth::SignInRequest,
    dto::auth::SignUpRequest, rest::error::Error,
};
use anyhow::Result;
use axum::{Json, extract::State, http::StatusCode};
use validator::Validate;

#[axum::debug_handler]
#[tracing::instrument(skip(state, req))]
pub async fn sign_up(
    State(state): State<AppContainer<'static>>,
    Json(req): Json<SignUpRequest>,
) -> Result<(StatusCode, Json<AuthTokens>), Error> {
    req.validate()?;

    let tokens = state.auth_service().await.sign_up(req.into()).await?;
    Ok((StatusCode::CREATED, Json(tokens)))
}

#[tracing::instrument(skip(state, req))]
pub async fn sign_in(
    State(state): State<AppContainer<'static>>,
    Json(req): Json<SignInRequest>,
) -> Result<(StatusCode, Json<AuthTokens>), Error> {
    req.validate()?;

    let tokens = state.auth_service().await.sign_in(req).await?;

    Ok((StatusCode::OK, Json(tokens)))
}
