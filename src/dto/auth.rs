use crate::domain::{SignInData, SignUpData};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(utoipa::ToSchema))]
pub struct SignInRequest {
    #[validate(email, length(min = 3, max = 256))]
    pub email: String,

    #[validate(length(min = 8, max = 64))]
    pub password: String,
}

impl Into<SignInData> for SignInRequest {
    fn into(self) -> SignInData {
        let Self { email, password } = self;

        SignInData {
            email,
            password: SecretString::from(password),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate)]
#[cfg_attr(feature = "ssr", derive(utoipa::ToSchema))]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessTokenClaims {
    pub exp: i64,
    pub iat: i64,
    pub id: Uuid,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ssr", derive(utoipa::ToSchema))]
pub struct AuthTokens {
    pub access_token: String,
    //TODO: refresh_token
}
