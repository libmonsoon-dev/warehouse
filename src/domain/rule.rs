use uuid::Uuid;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ssr", derive(diesel_derive_enum::DbEnum))]
#[cfg_attr(
    feature = "ssr",
    db_enum(
        existing_type_path = "crate::repository::postgresql::schema::sql_types::ResourceAction"
    )
)]
pub enum ResourceAction {
    Create,
    Read,
    List,
    Update,
    Delete,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ssr", derive(diesel_derive_enum::DbEnum))]
#[cfg_attr(
    feature = "ssr",
    db_enum(existing_type_path = "crate::repository::postgresql::schema::sql_types::ResourceType")
)]
pub enum ResourceType {
    User,
    Role,
    UserRole,
    Rule,
    RoleRule,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ssr", derive(diesel_derive_enum::DbEnum))]
#[cfg_attr(
    feature = "ssr",
    db_enum(existing_type_path = "crate::repository::postgresql::schema::sql_types::RuleEffect")
)]
pub enum RuleEffect {
    Allow,
    Deny,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "ssr",
    derive(diesel::Queryable, diesel::Selectable, diesel::Insertable)
)]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::repository::postgresql::schema::rules))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct Rule {
    pub id: Uuid,
    pub action: ResourceAction,
    pub resource_type: ResourceType,
    pub effect: RuleEffect,
}
