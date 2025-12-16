use secrecy::SecretString;
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: SecretString,
}

#[cfg_attr(
    feature = "ssr",
    derive(diesel::Queryable, diesel::Selectable, diesel::Insertable)
)]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::repository::postgresql::schema::user_roles))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct UserRole {
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub assigned_by: Option<Uuid>,
}
