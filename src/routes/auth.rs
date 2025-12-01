use crate::{
    dependency::AppContainer,
    domain,
    routes::error::{HttpError, RepositoryError},
    routes::user_repository::UserRepository,
    telemetry::spawn_blocking_with_tracing,
};
use anyhow::{Context, Result};
use argon2::{
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
    password_hash::SaltString, password_hash::rand_core::OsRng,
};
use axum::{Json, extract::State, http::StatusCode};
use chrono::{Duration, Utc};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthTokens {
    pub access_token: String,
    //TODO: refresh_token
}

#[axum::debug_handler]
#[tracing::instrument(skip(state, req))]
pub async fn sign_up(
    State(state): State<AppContainer<'static>>,
    Json(req): Json<SignUpRequest>,
) -> Result<(StatusCode, Json<AuthTokens>), HttpError> {
    req.validate()?;

    let tokens = state.auth_service().await.sign_up(req.into()).await?;
    Ok((StatusCode::CREATED, Json(tokens)))
}

#[tracing::instrument(skip(state, req))]
pub async fn sign_in(
    State(state): State<AppContainer<'static>>,
    Json(req): Json<SignInRequest>,
) -> Result<(StatusCode, Json<AuthTokens>), HttpError> {
    req.validate()?;

    let tokens = state.auth_service().await.sign_in(req).await?;

    Ok((StatusCode::OK, Json(tokens)))
}

#[derive(Deserialize, Validate)]
pub struct SignInRequest {
    #[validate(email, length(min = 3, max = 256))]
    pub email: String,

    #[validate(length(min = 8, max = 64))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct SignUpRequest {
    #[validate(length(min = 3, max = 256))]
    pub first_name: String,

    #[validate(length(min = 3, max = 256))]
    pub last_name: String,

    #[validate(email, length(min = 3, max = 256))]
    pub email: String,

    #[validate(length(min = 8, max = 64))]
    pub password: String,
}

impl Into<SignUpData> for SignUpRequest {
    fn into(self) -> SignUpData {
        let Self {
            first_name,
            last_name,
            email,
            password,
        } = self;

        SignUpData {
            first_name,
            last_name,
            email,
            password: SecretString::from(password),
        }
    }
}

#[derive(Clone)]
pub struct SignUpData {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: SecretString,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenClaims {
    pub exp: usize,
    pub iat: usize,
    pub id: Uuid,
    pub email: String,
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub struct AuthService {
    jwt_secret: SecretString,
    user_repository: Arc<dyn UserRepository>,
}

impl AuthService {
    pub fn new(jwt_secret: SecretString, user_repository: Arc<dyn UserRepository>) -> Self {
        Self {
            jwt_secret,
            user_repository,
        }
    }

    #[tracing::instrument(skip(self, args))]
    pub async fn sign_up(&self, args: SignUpData) -> Result<AuthTokens> {
        let mut user = domain::User {
            id: Uuid::new_v4(),
            first_name: args.first_name,
            last_name: args.last_name,
            email: args.email,
            password_hash: spawn_blocking_with_tracing(move || {
                compute_password_hash(args.password)
            })
            .await?
            .context("Failed to hash password")?,
        };

        self.user_repository
            .create(&mut user)
            .await
            .context("Failed to create user")?;

        let access_token = self.encode_access_jwt(&user)?;
        Ok(AuthTokens { access_token })
    }

    #[tracing::instrument(skip(self, _args))]
    async fn sign_in(&self, _args: SignInRequest) -> Result<AuthTokens> {
        todo!()
    }

    #[tracing::instrument(skip(self, credentials))]
    async fn _validate_credentials(&self, credentials: SignInRequest) -> Result<Uuid, AuthError> {
        let mut user_id = None;
        let mut expected_password_hash = SecretString::from(
            "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno",
        );

        match self.user_repository.get_by_email(&credentials.email).await {
            Ok(user) => {
                user_id = Some(user.id);
                expected_password_hash = user.password_hash;
            }
            Err(err) => 'errors: {
                for cause in err.chain() {
                    if let Some(repo_error) = cause.downcast_ref::<RepositoryError>() {
                        if matches!(repo_error, RepositoryError::NotFound) {
                            break 'errors;
                        }
                    }
                }

                return Err(AuthError::UnexpectedError(err));
            }
        };

        spawn_blocking_with_tracing(move || {
            verify_password_hash(expected_password_hash, credentials.password)
        })
        .await
        .context("Failed to spawn blocking task.")??;

        user_id
            .ok_or_else(|| anyhow::anyhow!("Unknown username."))
            .map_err(AuthError::InvalidCredentials)
    }

    #[tracing::instrument(skip(self, password))]
    pub async fn change_password(&self, user_id: Uuid, password: SecretString) -> Result<()> {
        let password_hash = spawn_blocking_with_tracing(move || compute_password_hash(password))
            .await?
            .context("Failed to hash password")?;

        self.user_repository
            .update_password_hash(user_id, &password_hash)
            .await
            .context("Failed to change user's password in the database.")?;
        Ok(())
    }

    #[tracing::instrument(skip(self, user), fields(id = %user.id))]
    pub fn encode_access_jwt(&self, user: &domain::User) -> Result<String> {
        let now = Utc::now();
        let expire = Duration::hours(24);
        let exp: usize = (now + expire).timestamp() as usize;
        let iat: usize = now.timestamp() as usize;
        let claim = AccessTokenClaims {
            iat,
            exp,
            id: user.id,
            email: user.email.to_owned(),
        };

        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claim,
            &jsonwebtoken::EncodingKey::from_secret(self.jwt_secret.expose_secret().as_ref()),
        )
        .context("Failed to encode access token.")
    }

    #[tracing::instrument(skip(self, token))]
    pub fn decode_access_jwt(
        &self,
        token: String,
    ) -> Result<jsonwebtoken::TokenData<AccessTokenClaims>> {
        jsonwebtoken::decode(
            &token,
            &jsonwebtoken::DecodingKey::from_secret(self.jwt_secret.expose_secret().as_ref()),
            &jsonwebtoken::Validation::default(),
        )
        .context("Failed to decode token.")
    }
}

#[allow(dead_code)] //false positive
#[tracing::instrument(skip(expected_password_hash, password_candidate))]
fn verify_password_hash(
    expected_password_hash: SecretString,
    password_candidate: String, //TODO: SecretString,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse hash in PHC string format.")?;

    new_argon()
        .verify_password(
            password_candidate //TODO:.expose_secret()
                .as_bytes(),
            &expected_password_hash,
        )
        .context("Invalid password.")
        .map_err(AuthError::InvalidCredentials)
}

fn compute_password_hash(password: SecretString) -> Result<SecretString> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = new_argon()
        .hash_password(password.expose_secret().as_bytes(), &salt)?
        .to_string();
    Ok(SecretString::from(password_hash))
}

fn new_argon() -> Argon2<'static> {
    Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
}
