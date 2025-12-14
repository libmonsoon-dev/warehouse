use crate::domain;
use diesel::{Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use uuid::Uuid;

#[derive(DbEnum, Debug)]
#[db_enum(existing_type_path = "crate::repository::postgresql::schema::sql_types::ResourceAction")]
pub enum ResourceAction {
    Create,
    Read,
    List,
    Update,
    Delete,
}

#[derive(DbEnum, Debug)]
#[db_enum(existing_type_path = "crate::repository::postgresql::schema::sql_types::ResourceType")]
pub enum ResourceType {
    User,
    Role,
    UserRole,
    Rule,
    RoleRule,
}

#[derive(DbEnum, Debug)]
#[db_enum(existing_type_path = "crate::repository::postgresql::schema::sql_types::RuleEffect")]
pub enum RuleEffect {
    Allow,
    Deny,
}

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
