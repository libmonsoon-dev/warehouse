use crate::contract::repository::{BridgeRepository, Repository};
use crate::domain;

#[async_trait::async_trait]
pub trait RoleRepository: Repository<domain::Role> {}

#[async_trait::async_trait]
pub trait RoleRuleRepository: BridgeRepository<domain::RoleRule> {}
