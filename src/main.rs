use tokio::net::TcpListener;
use warehouse::config::get_configuration;
use warehouse::dependency::AppContainer;
use warehouse::server;
use warehouse::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("warehouse".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let conf = get_configuration().expect("Failed to read configuration");

    let listener = TcpListener::bind(format!("0.0.0.0:{}", conf.server.port))
        .await
        .expect("Failed to bind");

    let dependency = AppContainer::new(conf);

    server::run(dependency, listener).await;
}
