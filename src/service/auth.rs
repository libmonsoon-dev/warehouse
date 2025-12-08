use crate::{
    contract::repository::error::RepositoryError,
    contract::repository::user::UserRepository,
    domain::{SignInData, SignUpData, User},
    dto::{AccessTokenClaims, AuthTokens},
    telemetry::spawn_blocking_with_tracing,
};
use anyhow::{Context, Result};
use argon2::{
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
    password_hash::SaltString, password_hash::rand_core::OsRng,
};
use chrono::{Duration, Utc};
use secrecy::{ExposeSecret, SecretString};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub struct AuthService {
    jwt_secret: SecretString,
    user_repository: Box<dyn UserRepository>,
}

impl AuthService {
    pub fn new(jwt_secret: SecretString, user_repository: Box<dyn UserRepository>) -> Self {
        Self {
            jwt_secret,
            user_repository,
        }
    }

    #[tracing::instrument(skip(self, args))]
    pub async fn sign_up(&self, args: SignUpData) -> Result<AuthTokens> {
        let mut user = User {
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

    #[tracing::instrument(skip(self, args))]
    pub async fn sign_in(&self, args: SignInData) -> Result<AuthTokens> {
        let user = self.validate_credentials(args).await?;

        let access_token = self.encode_access_jwt(&user)?;
        Ok(AuthTokens { access_token })
    }

    #[tracing::instrument(skip(self, credentials))]
    async fn validate_credentials(&self, credentials: SignInData) -> Result<User, AuthError> {
        let mut user = None;
        let mut expected_password_hash = SecretString::from(
            "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno",
        );

        match self.user_repository.get_by_email(&credentials.email).await {
            Ok(u) => {
                expected_password_hash = u.password_hash.clone();
                user = Some(u);
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

        user.ok_or_else(|| anyhow::anyhow!("Unknown email."))
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
    pub fn encode_access_jwt(&self, user: &User) -> Result<String> {
        let now = Utc::now();
        let expire = Duration::hours(24);
        let exp = (now + expire).timestamp();
        let iat = now.timestamp();
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
    password_candidate: SecretString,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse hash in PHC string format.")?;

    new_argon()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
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
