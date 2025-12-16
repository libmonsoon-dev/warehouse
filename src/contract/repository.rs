use anyhow::Result;
use uuid::Uuid;

mod role;
mod rule;
mod user;

pub use role::*;
pub use rule::*;
pub use user::*;

#[async_trait::async_trait]
pub trait Repository<T>: Send + Sync {
    async fn create(&self, val: T) -> Result<T>;

    async fn get_by_id(&self, id: Uuid) -> Result<T>;
}

#[async_trait::async_trait]
pub trait BridgeRepository<T>: Send + Sync {
    async fn create(&self, val: T) -> Result<T>;
}
