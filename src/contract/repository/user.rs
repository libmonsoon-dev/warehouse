use crate::contract::repository::Repository;
use crate::domain;
use anyhow::Result;
use secrecy::SecretString;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait UserRepository: Repository<domain::User> {
    async fn get_by_email(&self, email: &str) -> Result<domain::User>;
    async fn update_password_hash(&self, user_id: Uuid, password_hash: &SecretString)
    -> Result<()>;
}
