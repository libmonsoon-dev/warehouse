use crate::contract::repository::error::RepositoryError;
use crate::{
    contract::repository::Repository, contract::repository::user::UserRepository, db, domain,
    repository::postgresql::models::User, repository::postgresql::schema::users,
};
use anyhow::{Context, Result, anyhow};
use diesel::{prelude::*, result::DatabaseErrorKind};
use diesel_async::RunQueryDsl;
use secrecy::{ExposeSecret, SecretString};
use std::sync::Arc;
use uuid::Uuid;

impl From<User> for domain::User {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            password_hash: user.password_hash.into(),
        }
    }
}

pub struct PostgresUserRepo {
    pool: Arc<db::Pool>,
}

impl PostgresUserRepo {
    pub fn new(pool: Arc<db::Pool>) -> Self {
        PostgresUserRepo { pool }
    }
}

#[async_trait::async_trait]
impl Repository<domain::User> for PostgresUserRepo {
    #[tracing::instrument(skip(self, user))]
    async fn create(&self, user: &mut domain::User) -> Result<()> {
        let mut conn = self.pool.get().await.context("DB connection failure")?;

        let model = User {
            id: user.id,
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
            password_hash: user.password_hash.expose_secret().into(),
        };

        let insert_result = diesel::insert_into(users::table)
            .values(model)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .await;

        match insert_result {
            Err(diesel::result::Error::DatabaseError(kind, info)) => match &kind {
                DatabaseErrorKind::UniqueViolation => {
                    Err(RepositoryError::Exists(anyhow!(info.message().to_string())).into())
                }
                _ => Err(anyhow!(info.message().to_string())),
            },
            Err(err) => Err(err.into()),
            Ok(model) => {
                user.id = model.id;
                user.first_name = user.first_name.clone();
                user.last_name = user.last_name.clone();
                user.email = user.email.clone();
                user.password_hash = user.password_hash.clone();

                Ok(())
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn get_by_id(&self, id: Uuid) -> Result<domain::User> {
        let mut conn = self.pool.get().await.context("DB connection failure")?;

        let model = users::table
            .find(id)
            .select(User::as_select())
            .first(&mut conn)
            .await
            .optional()?;

        match model {
            Some(model) => Ok(model.into()),
            None => Err(RepositoryError::NotFound.into()),
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepo {
    #[tracing::instrument(skip(self))]
    async fn get_by_email(&self, _email: &str) -> Result<domain::User> {
        todo!()
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
