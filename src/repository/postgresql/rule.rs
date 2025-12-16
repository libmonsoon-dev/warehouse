use crate::contract::repository::{Repository, RuleRepository};
use crate::repository::postgresql::map_diesel_error;
use crate::repository::postgresql::schema::rules;
use crate::{db, domain};
use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

pub struct PostgresRuleRepository {
    pool: db::Pool,
}

impl PostgresRuleRepository {
    pub fn new(pool: db::Pool) -> Self {
        Self { pool }
    }

    async fn get_connection(&self) -> Result<db::Connection> {
        self.pool.get().await.context("get connection")
    }
}

#[async_trait::async_trait]
impl Repository<domain::Rule> for PostgresRuleRepository {
    #[tracing::instrument(skip(self, val))]
    async fn create(&self, val: domain::Rule) -> Result<domain::Rule> {
        diesel::insert_into(rules::table)
            .values(val)
            .returning(domain::Rule::as_returning())
            .get_result(&mut self.get_connection().await?)
            .await
            .map_err(map_diesel_error)
    }

    #[tracing::instrument(skip(self))]
    async fn get_by_id(&self, id: Uuid) -> Result<domain::Rule> {
        rules::table
            .find(id)
            .select(domain::Rule::as_select())
            .first(&mut self.get_connection().await?)
            .await
            .map_err(map_diesel_error)
    }
}

#[async_trait::async_trait]
impl RuleRepository for PostgresRuleRepository {}
