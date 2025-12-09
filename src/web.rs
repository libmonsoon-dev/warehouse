pub mod app;
mod page;
mod component;
mod error;
mod client;
#[cfg(feature = "ssr")]
mod middleware;
mod utils;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::web::app::App;

    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
