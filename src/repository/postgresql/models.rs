use crate::domain;
use diesel::{Insertable, Queryable, Selectable};
use secrecy::ExposeSecret;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::repository::postgresql::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
}

impl From<User> for domain::User {
    fn from(user: User) -> Self {
        let User {
            id,
            first_name,
            last_name,
            email,
            password_hash,
        } = user;

        Self {
            id,
            first_name,
            last_name,
            email,
            password_hash: password_hash.into(),
        }
    }
}

impl From<domain::User> for User {
    fn from(user: domain::User) -> Self {
        let domain::User {
            id,
            first_name,
            last_name,
            email,
            password_hash,
        } = user;

        Self {
            id,
            first_name,
            last_name,
            email,
            password_hash: password_hash.expose_secret().into(),
        }
    }
}
