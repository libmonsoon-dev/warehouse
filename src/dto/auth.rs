use crate::domain::auth::SignUpData;
use secrecy::SecretString;
use serde::Deserialize;
use validator::Validate;

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
