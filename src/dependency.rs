use crate::{
    config::Config, contract::repository::user::UserRepository, db,
    repository::postgresql::user::PostgresUserRepo, service::auth::AuthService,
};
use despatma::dependency_container;

#[dependency_container(pub)]
impl AppContainer {
    fn new(config: Config) {}

    #[Singleton]
    async fn db_pool(&self, config: &Config) -> db::Pool {
        db::connect(&config.database).await
    }

    async fn user_repository(&self, db_pool: &db::Pool) -> Box<dyn UserRepository> {
        Box::new(PostgresUserRepo::new(db_pool.clone()))
    }

    async fn auth_service(
        &self,
        config: &Config,
        user_repository: Box<dyn UserRepository>,
    ) -> AuthService {
        AuthService::new(config.server.jwtsecret.clone(), user_repository)
    }
}
