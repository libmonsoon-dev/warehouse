use crate::config::Config;
use crate::contract::repository::{
    RoleRepository, RoleRuleRepository, RuleRepository, UserRepository, UserRoleRepository,
};
use crate::db;
use crate::repository::postgresql::{
    PostgresRoleRepository, PostgresRoleRuleRepository, PostgresRuleRepository,
    PostgresUserRepository, PostgresUserRoleRepository,
};
use crate::service::auth::AuthService;
use despatma::dependency_container;

#[dependency_container(pub)]
impl AppContainer {
    fn new(config: Config) {}

    #[Singleton]
    async fn db_pool(&self, config: &Config) -> db::Pool {
        db::connect(&config.database).await
    }

    async fn user_repository(&self, db_pool: &db::Pool) -> Box<dyn UserRepository> {
        Box::new(PostgresUserRepository::new(db_pool.clone()))
    }

    async fn user_role_repository(&self, db_pool: &db::Pool) -> Box<dyn UserRoleRepository> {
        Box::new(PostgresUserRoleRepository::new(db_pool.clone()))
    }

    async fn role_repository(&self, db_pool: &db::Pool) -> Box<dyn RoleRepository> {
        Box::new(PostgresRoleRepository::new(db_pool.clone()))
    }

    async fn role_rule_repository(&self, db_pool: &db::Pool) -> Box<dyn RoleRuleRepository> {
        Box::new(PostgresRoleRuleRepository::new(db_pool.clone()))
    }

    async fn rule_repository(&self, db_pool: &db::Pool) -> Box<dyn RuleRepository> {
        Box::new(PostgresRuleRepository::new(db_pool.clone()))
    }

    #[Singleton]
    async fn auth_service(
        &self,
        config: &Config,
        user_repository: Box<dyn UserRepository>,
    ) -> AuthService {
        AuthService::new(config.server.jwtsecret.clone(), user_repository)
    }
}
