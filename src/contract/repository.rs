use anyhow::Result;
use uuid::Uuid;

pub mod error;
pub mod user;

#[async_trait::async_trait]
pub trait Repository<T>: Send + Sync {
    async fn create(&self, val: &mut T) -> Result<()>;

    async fn get_by_id(&self, id: Uuid) -> Result<T>;
}
