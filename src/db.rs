use crate::config::DatabaseConfig;
use deadpool::managed::Object;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use secrecy::ExposeSecret;

pub type ConnectionManager = AsyncDieselConnectionManager<AsyncPgConnection>;

pub type Connection = Object<ConnectionManager>;

pub type Pool = deadpool::managed::Pool<ConnectionManager, Connection>;

pub async fn connect(conf: &DatabaseConfig) -> Pool {
    let conn_manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(
        conf.connection_string().expose_secret(),
    );

    Pool::builder(conn_manager)
        .build()
        .expect("Failed to build connection pool")
}
