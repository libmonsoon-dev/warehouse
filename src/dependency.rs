use crate::{
    config::Config,
    db,
    routes::auth::AuthService,
    routes::user_repository::{PostgresUserRepo, UserRepository},
};
use despatma::dependency_container;
use std::sync::Arc;

#[dependency_container(pub)]
impl AppContainer {
    fn new(config: Config) {}

    #[Singleton]
    async fn db_pool(&self, config: &Config) -> Arc<db::Pool> {
        Arc::new(db::connect(&config.database).await)
    }

    async fn user_repository(&self, db_pool: &Arc<db::Pool>) -> Arc<dyn UserRepository> {
        Arc::new(PostgresUserRepo::new(db_pool.clone()))
    }

    async fn auth_service(
        &self,
        config: &Config,
        user_repository: &Arc<dyn UserRepository>,
    ) -> Arc<AuthService> {
        Arc::new(AuthService::new(
            config.server.jwtsecret.clone(),
            user_repository.clone(),
        ))
    }
}
