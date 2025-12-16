use uuid::Uuid;

#[cfg_attr(
    feature = "ssr",
    derive(diesel::Queryable, diesel::Selectable, diesel::Insertable)
)]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::repository::postgresql::schema::roles))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[cfg_attr(
    feature = "ssr",
    derive(diesel::Queryable, diesel::Selectable, diesel::Insertable)
)]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::repository::postgresql::schema::role_rules))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct RoleRule {
    pub role_id: Uuid,
    pub rule_id: Uuid,
    pub assigned_by: Option<Uuid>,
}
