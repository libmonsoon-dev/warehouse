use crate::contract::repository::{
    BridgeRepository, Repository, RoleRepository, RoleRuleRepository,
};
use crate::repository::postgresql::map_diesel_error;
use crate::repository::postgresql::schema::{role_rules, roles};
use crate::{db, domain};
use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

pub struct PostgresRoleRepository {
    pool: db::Pool,
}

impl PostgresRoleRepository {
    pub fn new(pool: db::Pool) -> Self {
        Self { pool }
    }

    async fn get_connection(&self) -> Result<db::Connection> {
        self.pool.get().await.context("get connection")
    }
}

#[async_trait::async_trait]
impl Repository<domain::Role> for PostgresRoleRepository {
    #[tracing::instrument(skip(self, val))]
    async fn create(&self, val: domain::Role) -> Result<domain::Role> {
        diesel::insert_into(roles::table)
            .values(val)
            .returning(domain::Role::as_returning())
            .get_result(&mut self.get_connection().await?)
            .await
            .map_err(map_diesel_error)
    }

    #[tracing::instrument(skip(self))]
    async fn get_by_id(&self, id: Uuid) -> Result<domain::Role> {
        roles::table
            .find(id)
            .select(domain::Role::as_select())
            .first(&mut self.get_connection().await?)
            .await
            .map_err(map_diesel_error)
    }
}

#[async_trait::async_trait]
impl RoleRepository for PostgresRoleRepository {}

pub struct PostgresRoleRuleRepository {
    pool: db::Pool,
}

impl PostgresRoleRuleRepository {
    pub fn new(pool: db::Pool) -> Self {
        Self { pool }
    }

    async fn get_connection(&self) -> Result<db::Connection> {
        self.pool.get().await.context("get connection")
    }
}

#[async_trait::async_trait]
impl BridgeRepository<domain::RoleRule> for PostgresRoleRuleRepository {
    #[tracing::instrument(skip(self, val))]
    async fn create(&self, val: domain::RoleRule) -> Result<domain::RoleRule> {
        diesel::insert_into(role_rules::table)
            .values(val)
            .returning(domain::RoleRule::as_returning())
            .get_result(&mut self.get_connection().await?)
            .await
            .map_err(map_diesel_error)
    }
}

#[async_trait::async_trait]
impl RoleRuleRepository for PostgresRoleRuleRepository {}
