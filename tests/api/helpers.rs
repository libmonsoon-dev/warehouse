use diesel::Connection;
use diesel::pg::PgConnection;
use diesel::sql_query;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use reqwest::header::CONTENT_TYPE;
use reqwest::{Error, Response};
use secrecy::{ExposeSecret, SecretString};
use std::sync::LazyLock;
use tokio::net::TcpListener;
use uuid::Uuid;
use warehouse::config::{DatabaseConfig, get_configuration};
use warehouse::routes::auth::SignUpData;
use warehouse::server;
use warehouse::server::AppContainer;
use warehouse::telemetry::{get_subscriber, init_subscriber};

static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    pub admin: SignUpData,
    pub dependency: AppContainer<'static>,
}

impl TestApp {
    pub async fn sign_up(&self, body: String) -> Result<Response, Error> {
        reqwest::Client::new()
            .post(&format!("{}/api/v1/auth/sign-up", &self.address))
            .header(CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await
    }

    pub async fn health_check(self) -> Result<Response, Error> {
        reqwest::Client::new()
            .get(&format!("{}/api/v1/health-check", &self.address))
            .send()
            .await
    }
}

pub async fn spawn_app() -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    LazyLock::force(&TRACING);

    let listener = TcpListener::bind("0.0.0.0:0")
        .await
        .expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database = format!("test_{}", Uuid::new_v4().to_string());
    configure_database(&configuration.database).await;
    let dependency = AppContainer::new(configuration);

    let admin = SignUpData {
        first_name: "admin".to_string(),
        last_name: "admin".to_string(),
        email: "admin@warehouse.com".to_string(),
        password: SecretString::from("admin-pass"),
    };

    dependency
        .auth_service()
        .await
        .sign_up(admin.clone())
        .await
        .expect("Failed to create admin user");

    let server = server::run(dependency.clone(), listener);
    let _ = tokio::spawn(server);
    TestApp {
        address,
        admin,
        dependency,
    }
}

async fn configure_database(conf: &DatabaseConfig) {
    // Create database
    let maintenance_config = DatabaseConfig {
        database: "postgres".to_string(),
        username: "postgres".to_string(),
        password: SecretString::from("mysecretpassword".to_string()),
        ..conf.clone()
    };
    let connection =
        &mut AsyncPgConnection::establish(&maintenance_config.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres");

    sql_query(format!(r#"CREATE DATABASE "{}";"#, conf.database))
        .execute(connection)
        .await
        .expect("Failed to create database.");

    let connection_string = conf.connection_string();
    tokio::task::spawn_blocking(move || {
        let mut sync_conn = PgConnection::establish(connection_string.expose_secret())
            .expect("Filed to establish sync connection");
        run_migration(&mut sync_conn)
    })
    .await
    .expect("Filed to migrate database");
}

fn run_migration(conn: &mut impl MigrationHarness<diesel::pg::Pg>) {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run pending migrations.");
}
