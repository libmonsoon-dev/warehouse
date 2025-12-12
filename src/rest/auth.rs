use crate::{
    dto::{AuthTokens, SignInRequest, SignUpRequest},
    rest::error::ServerError,
    state::AppState,
};
use anyhow::Result;
use axum::{Json, extract::State, http::StatusCode};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use validator::Validate;

#[utoipa::path(post, path = "/sign-up", responses((status = OK, body = AuthTokens)), tag = crate::apidoc::AUTH_TAG)]
#[tracing::instrument(skip(state, req))]
pub async fn sign_up(
    State(state): State<AppState>,
    Json(req): Json<SignUpRequest>,
) -> Result<(StatusCode, Json<AuthTokens>), ServerError> {
    req.validate()?;

    let tokens = state
        .dependencies
        .auth_service()
        .await
        .sign_up(req.into())
        .await?;
    Ok((StatusCode::CREATED, Json(tokens)))
}

#[utoipa::path(post, path = "/sign-in", responses((status = OK, body = AuthTokens)), tag = crate::apidoc::AUTH_TAG)]
#[tracing::instrument(skip(state, req))]
pub async fn sign_in(
    State(state): State<AppState>,
    Json(req): Json<SignInRequest>,
) -> Result<(StatusCode, Json<AuthTokens>), ServerError> {
    req.validate()?;

    let tokens = state
        .dependencies
        .auth_service()
        .await
        .sign_in(req.into())
        .await?;
    Ok((StatusCode::OK, Json(tokens)))
}

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(sign_up))
        .routes(routes!(sign_in))
}
