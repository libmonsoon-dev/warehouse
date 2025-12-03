use crate::dependency::AppContainer;
use axum::extract::FromRef;
use leptos::config::LeptosOptions;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub dependencies: AppContainer<'static>,
    pub leptos_options: LeptosOptions,
}
