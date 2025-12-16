use crate::contract::repository::Repository;
use crate::domain;

#[async_trait::async_trait]
pub trait RuleRepository: Repository<domain::Rule> {}
