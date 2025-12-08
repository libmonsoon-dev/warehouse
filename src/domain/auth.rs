use secrecy::SecretString;

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
