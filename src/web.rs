pub mod app;
mod client;
mod component;
mod error;
#[cfg(feature = "ssr")]
mod middleware;
mod page;
mod utils;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::{
        telemetry::{get_subscriber, init_subscriber},
        web::app::App,
    };

    console_error_panic_hook::set_once();
    let subscriber = get_subscriber();
    init_subscriber(subscriber);
    leptos::mount::hydrate_body(App);
}
