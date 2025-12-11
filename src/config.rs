use anyhow::{Context, Error, anyhow};
use config::Environment;
use dotenvy::dotenv;
use secrecy::{ExposeSecret, SecretString};
use std::collections::HashMap;
use std::env;
use url::Url;

#[derive(serde::Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

#[derive(serde::Deserialize, Clone)]
pub struct ServerConfig {
    pub jwtsecret: SecretString,
}

#[derive(serde::Deserialize, Clone, Default)]
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

impl TryFrom<&str> for DatabaseConfig {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let url = Url::parse(value).context("parse database url")?;

        let path = url.path();
        if path.len() < 2 {
            return Err(anyhow!("missing database"));
        }

        Ok(Self {
            username: url.username().to_owned(),
            password: url
                .password()
                .ok_or(anyhow!("missing password"))?
                .to_owned()
                .into(),
            port: url.port().unwrap_or(5432),
            host: url.host().ok_or(anyhow!("missing host"))?.to_string(),
            database: path[1..path.len()].to_owned(),
        })
    }
}

impl Into<config::Value> for DatabaseConfig {
    fn into(self) -> config::Value {
        let table = HashMap::<String, config::Value>::from([
            ("username".to_string(), self.username.clone().into()),
            ("password".to_string(), self.password.expose_secret().into()),
            ("port".to_string(), self.port.into()),
            ("host".to_string(), self.host.clone().into()),
            ("database".to_string(), self.database.clone().into()),
        ]);

        config::Value::new(
            Some(&self.connection_string().expose_secret().to_string()),
            config::ValueKind::Table(table),
        )
    }
}

pub fn get_configuration() -> Result<Config, anyhow::Error> {
    dotenv().context("load .env file")?;

    let settings = config::Config::builder()
        .set_override(
            "database",
            DatabaseConfig::try_from(
                env::var("DATABASE_URL")
                    .context("read database url")?
                    .as_str(),
            )
            .context("parse database url")?,
        )
        .context("override database config")?
        .add_source(Environment::with_prefix("warehouse").separator("_"))
        .build()?;

    settings
        .try_deserialize::<Config>()
        .context("deserialize configuration")
}
