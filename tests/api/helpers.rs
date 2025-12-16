use anyhow::{Context, Result};
use diesel::Connection;
use diesel::pg::PgConnection;
use diesel::sql_query;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use leptos::config::LeptosOptions;
use reqwest::Response;
use reqwest::header::CONTENT_TYPE;
use secrecy::{ExposeSecret, SecretString};
use std::sync::LazyLock;
use tokio::net::TcpListener;
use uuid::Uuid;
use warehouse::config::{Config, DatabaseConfig};
use warehouse::domain::{Role, RoleRule, Rule, User, UserRole};
use warehouse::service::auth::compute_password_hash;
use warehouse::{
    config::get_configuration,
    dependency::AppContainer,
    domain, server,
    telemetry::{get_subscriber, init_subscriber},
};

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

pub struct TestApp<'a> {
    pub address: String,
    pub dependency: AppContainer<'a>,
    pub data: TestData,
}

pub struct TestData {
    pub admin: domain::SignUpData,
    pub admin_id: Uuid,
}

impl<'a> TestApp<'a> {
    pub async fn sign_up(&self, body: String) -> Result<Response, reqwest::Error> {
        reqwest::Client::new()
            .post(&format!("{}/api/v1/auth/sign-up", &self.address))
            .header(CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await
    }

    pub async fn sign_in(&self, body: String) -> Result<Response, reqwest::Error> {
        reqwest::Client::new()
            .post(&format!("{}/api/v1/auth/sign-in", &self.address))
            .header(CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await
    }

    pub async fn health_check(self) -> Result<Response, reqwest::Error> {
        reqwest::Client::new()
            .get(&format!("{}/api/v1/health-check", &self.address))
            .send()
            .await
    }
}

pub async fn spawn_app<'a>() -> TestApp<'a> {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    LazyLock::force(&TRACING);

    let listener = TcpListener::bind("0.0.0.0:0")
        .await
        .expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let configuration = get_configuration().expect("Failed to read configuration");
    let (dependency, data) = setup_test_database(configuration)
        .await
        .expect("Failed to setup database");

    let leptos_options = LeptosOptions::builder().output_name("test").build();

    let server = server::run(leptos_options, dependency.clone(), listener);
    let _ = tokio::spawn(server);
    TestApp {
        address,
        data,
        dependency,
    }
}

async fn setup_test_database<'a>(mut config: Config) -> Result<(AppContainer<'a>, TestData)> {
    config.database.database = format!("test_{}", Uuid::new_v4().to_string());
    configure_database(&config.database).await?;

    let dependencies = AppContainer::new(config);

    let data = populate_database(&dependencies).await?;

    Ok((dependencies, data))
}

async fn configure_database(conf: &DatabaseConfig) -> Result<()> {
    // Create database
    let maintenance_config = DatabaseConfig {
        database: "postgres".to_string(),
        ..conf.clone()
    };

    let connection =
        &mut AsyncPgConnection::establish(&maintenance_config.connection_string().expose_secret())
            .await
            .context("Failed to connect to Postgres")?;

    sql_query(format!(r#"CREATE DATABASE "{}";"#, conf.database))
        .execute(connection)
        .await
        .context("Failed to create database.")?;

    let connection_string = conf.connection_string();
    tokio::task::spawn_blocking(move || {
        let mut sync_conn = PgConnection::establish(connection_string.expose_secret())
            .expect("Filed to establish sync connection");
        run_migration(&mut sync_conn)
    })
    .await
    .context("Filed to migrate database")
}

fn run_migration(conn: &mut impl MigrationHarness<diesel::pg::Pg>) {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run pending migrations.");
}

async fn populate_database<'a>(dependencies: &AppContainer<'a>) -> Result<TestData> {
    let admin_sign_up_data = domain::SignUpData {
        first_name: "admin".to_string(),
        last_name: "admin".to_string(),
        email: "admin@warehouse.com".to_string(),
        password: SecretString::from("admin-pass"),
    };

    let admin = User {
        id: Uuid::new_v4(),
        first_name: admin_sign_up_data.first_name.clone(),
        last_name: admin_sign_up_data.last_name.clone(),
        email: admin_sign_up_data.email.clone(),
        password_hash: compute_password_hash(admin_sign_up_data.password.clone())
            .context("Failed to hash password")?,
    };

    let admin = dependencies
        .user_repository()
        .await
        .create(admin)
        .await
        .context("Failed to create user")?;

    let allow_create_role = Rule {
        id: Uuid::new_v4(),
        action: domain::ResourceAction::Create,
        resource_type: domain::ResourceType::Role,
        effect: domain::RuleEffect::Allow,
    };

    let allow_create_user_role = Rule {
        id: Uuid::new_v4(),
        action: domain::ResourceAction::Create,
        resource_type: domain::ResourceType::UserRole,
        effect: domain::RuleEffect::Allow,
    };

    let allow_create_rule = Rule {
        id: Uuid::new_v4(),
        action: domain::ResourceAction::Create,
        resource_type: domain::ResourceType::Rule,
        effect: domain::RuleEffect::Allow,
    };

    let allow_create_role_rule = Rule {
        id: Uuid::new_v4(),
        action: domain::ResourceAction::Create,
        resource_type: domain::ResourceType::RoleRule,
        effect: domain::RuleEffect::Allow,
    };

    let root_rules = vec![
        allow_create_role,
        allow_create_user_role,
        allow_create_rule,
        allow_create_role_rule,
    ];

    for rule in root_rules.iter().cloned() {
        dependencies.rule_repository().await.create(rule).await?;
    }

    let root_role = dependencies
        .role_repository()
        .await
        .create(Role {
            id: Uuid::new_v4(),
            name: "root".to_string(),
            description: None,
        })
        .await?;

    for rule in root_rules {
        dependencies
            .role_rule_repository()
            .await
            .create(RoleRule {
                role_id: root_role.id,
                rule_id: rule.id,
                assigned_by: None,
            })
            .await?;
    }

    dependencies
        .user_role_repository()
        .await
        .create(UserRole {
            user_id: admin.id,
            role_id: root_role.id,
            assigned_by: None,
        })
        .await?;

    Ok(TestData {
        admin: admin_sign_up_data,
        admin_id: admin.id,
    })
}
