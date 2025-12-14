// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "resource_action"))]
    pub struct ResourceAction;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "resource_type"))]
    pub struct ResourceType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "rule_effect"))]
    pub struct RuleEffect;
}

diesel::table! {
    role_rules (role_id, rule_id) {
        role_id -> Uuid,
        rule_id -> Uuid,
        assigned_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ResourceAction;
    use super::sql_types::ResourceType;
    use super::sql_types::RuleEffect;

    rules (id) {
        id -> Uuid,
        action -> ResourceAction,
        resource_type -> ResourceType,
        effect -> RuleEffect,
    }
}

diesel::table! {
    user_roles (user_id, role_id) {
        user_id -> Uuid,
        role_id -> Uuid,
        assigned_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 256]
        first_name -> Varchar,
        #[max_length = 256]
        last_name -> Varchar,
        #[max_length = 256]
        email -> Varchar,
        #[max_length = 256]
        password_hash -> Varchar,
    }
}

diesel::joinable!(role_rules -> roles (role_id));
diesel::joinable!(role_rules -> rules (rule_id));
diesel::joinable!(user_roles -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(role_rules, roles, rules, user_roles, users,);
