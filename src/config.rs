use config::Environment;
use dotenvy::dotenv;
use secrecy::{ExposeSecret, SecretString};

#[derive(serde::Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

#[derive(serde::Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub jwtsecret: SecretString,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: SecretString,
    pub port: u16,
    pub host: String,
    pub database: String,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> SecretString {
        SecretString::from(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database
        ))
    }
}

pub fn get_configuration() -> Result<Config, config::ConfigError> {
    dotenv().ok();

    let settings = config::Config::builder()
        .add_source(Environment::with_prefix("warehouse").separator("_"))
        .build()?;
    settings.try_deserialize::<Config>()
}
