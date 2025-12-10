#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use tokio::net::TcpListener;
    use warehouse::config::get_configuration;
    use warehouse::dependency::AppContainer;
    use warehouse::server;
    use warehouse::telemetry::{get_subscriber, init_subscriber};

    let subscriber = get_subscriber("warehouse".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let conf = get_configuration().expect("Failed to read configuration");

    let dependency = AppContainer::new(conf);
    let leptos_options = leptos::config::get_configuration(None)
        .expect("leptos configuration")
        .leptos_options;

    let listener = TcpListener::bind(leptos_options.site_addr)
        .await
        .expect("Failed to bind");

    server::run(leptos_options, dependency, listener).await;
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see web.rs for hydration function instead
}
