use secrecy::SecretString;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct SignUpData {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: SecretString,
}

pub struct SignInData {
    pub email: String,
    pub password: SecretString,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AuthTokens {
    pub access_token: String,
    //TODO: refresh_token
}
