use crate::contract::repository::{
    BridgeRepository, Repository, UserRepository, UserRoleRepository,
};
use crate::db;
use crate::domain;
use crate::repository::postgresql::map_diesel_error;
use crate::repository::postgresql::models::User;
use crate::repository::postgresql::schema::{user_roles, users};
use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use secrecy::SecretString;
use uuid::Uuid;

pub struct PostgresUserRepository {
    pool: db::Pool,
}

impl PostgresUserRepository {
    pub fn new(pool: db::Pool) -> Self {
        Self { pool }
    }

    async fn get_connection(&self) -> Result<db::Connection> {
        self.pool.get().await.context("get connection")
    }
}

#[async_trait::async_trait]
impl Repository<domain::User> for PostgresUserRepository {
    #[tracing::instrument(skip(self, user))]
    async fn create(&self, user: domain::User) -> Result<domain::User> {
        diesel::insert_into(users::table)
            .values(User::from(user))
            .returning(User::as_returning())
            .get_result(&mut self.get_connection().await?)
            .await
            .map(domain::User::from)
            .map_err(map_diesel_error)
    }

    #[tracing::instrument(skip(self))]
    async fn get_by_id(&self, id: Uuid) -> Result<domain::User> {
        users::table
            .find(id)
            .select(User::as_select())
            .first(&mut self.get_connection().await?)
            .await
            .map(domain::User::from)
            .map_err(map_diesel_error)
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepository {
    #[tracing::instrument(skip(self))]
    async fn get_by_email(&self, email: &str) -> Result<domain::User> {
        users::table
            .select(User::as_select())
            .filter(users::email.eq(email))
            .first(&mut self.get_connection().await?)
            .await
            .map(domain::User::from)
            .map_err(map_diesel_error)
    }

    #[tracing::instrument(skip(self, _password_hash))]
    async fn update_password_hash(
        &self,
        _user_id: Uuid,
        _password_hash: &SecretString,
    ) -> Result<()> {
        todo!()
    }
}

pub struct PostgresUserRoleRepository {
    pool: db::Pool,
}

impl PostgresUserRoleRepository {
    pub fn new(pool: db::Pool) -> Self {
        Self { pool }
    }

    async fn get_connection(&self) -> Result<db::Connection> {
        self.pool.get().await.context("get connection")
    }
}

#[async_trait::async_trait]
impl BridgeRepository<domain::UserRole> for PostgresUserRoleRepository {
    #[tracing::instrument(skip(self, val))]
    async fn create(&self, val: domain::UserRole) -> Result<domain::UserRole> {
        diesel::insert_into(user_roles::table)
            .values(val)
            .returning(domain::UserRole::as_returning())
            .get_result(&mut self.get_connection().await?)
            .await
            .map_err(map_diesel_error)
    }
}

#[async_trait::async_trait]
impl UserRoleRepository for PostgresUserRoleRepository {}
