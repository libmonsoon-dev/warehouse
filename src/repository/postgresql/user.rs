use crate::contract::repository::Repository;
use crate::contract::repository::user::UserRepository;
use crate::db;
use crate::domain;
use crate::domain::RepositoryError;
use crate::repository::postgresql::models::User;
use crate::repository::postgresql::schema::users;
use anyhow::{Context, Result, anyhow};
use diesel::{prelude::*, result::DatabaseErrorKind};
use diesel_async::RunQueryDsl;
use secrecy::{ExposeSecret, SecretString};
use uuid::Uuid;

pub struct PostgresUserRepo {
    pool: db::Pool,
}

impl PostgresUserRepo {
    pub fn new(pool: db::Pool) -> Self {
        PostgresUserRepo { pool }
    }

    async fn get_connection(&self) -> Result<db::Connection> {
        self.pool.get().await.context("get connection")
    }
}

#[async_trait::async_trait]
impl Repository<domain::User> for PostgresUserRepo {
    #[tracing::instrument(skip(self, user))]
    async fn create(&self, user: &mut domain::User) -> Result<()> {
        let mut conn = self.get_connection().await?;

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
        let mut conn = self.get_connection().await?;

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
    async fn get_by_email(&self, email: &str) -> Result<domain::User> {
        let mut conn = self.get_connection().await?;

        let model = users::table
            .select(User::as_select())
            .filter(users::email.eq(email))
            .first(&mut conn)
            .await
            .optional()?;

        match model {
            Some(model) => Ok(model.into()),
            None => Err(RepositoryError::NotFound.into()),
        }
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
