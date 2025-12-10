use tracing::Subscriber;
use tracing::subscriber::set_global_default;
use tracing_log::LogTracer;

#[cfg(feature = "ssr")]
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Sync + Send
where
    Sink: for<'a> tracing_subscriber::fmt::MakeWriter<'a> + Send + Sync + 'static,
{
    use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
    use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

#[cfg(feature = "hydrate")]
pub fn get_subscriber() -> impl Subscriber {
    use tracing_subscriber_wasm::MakeConsoleWriter;

    tracing_subscriber::fmt()
        .with_writer(
            MakeConsoleWriter::default(),
        )
        .with_ansi(false)
        // For some reason, if we don't do this in the browser, we get
        // a runtime error.
        .without_time()
        .finish()
}

pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}

#[cfg(feature = "ssr")]
pub fn spawn_blocking_with_tracing<F, R>(f: F) -> tokio::task::JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}
