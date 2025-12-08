use crate::dto::AuthTokens;
use codee::string::JsonSerdeCodec;
use leptos::component;
use leptos::prelude::*;
use leptos::{IntoView, view};
use leptos_use::storage::{UseStorageOptions, use_local_storage, use_local_storage_with_options};

#[component]
pub fn Authorized(children: Children) -> impl IntoView {
    let (tokens, _, _) = use_auth_tokens();
    let navigate = leptos_router::hooks::use_navigate();

    Effect::new(move |_| {
        if let None = tokens.get() {
            navigate("/sign-in", Default::default());
        }
    });

    children()
}

#[component]
pub fn LogOutButton() -> impl IntoView {
    let (tokens, set_tokens, _) = use_delayed_auth_tokens();

    move || {
        tokens.get().map(|_| {
            view! { <button on:click=move |_| { set_tokens.set(None) }>Logout</button> }
        })
    }
}

pub fn use_auth_tokens() -> (
    Signal<Option<AuthTokens>>,
    WriteSignal<Option<AuthTokens>>,
    impl Fn() + Clone + Send + Sync,
) {
    use_local_storage::<Option<AuthTokens>, JsonSerdeCodec>(AUTH_TOKENS_LOCAL_STORAGE_KEY)
}

pub fn use_delayed_auth_tokens() -> (
    Signal<Option<AuthTokens>>,
    WriteSignal<Option<AuthTokens>>,
    impl Fn() + Clone + Send + Sync,
) {
    use_local_storage_with_options::<Option<AuthTokens>, JsonSerdeCodec>(
        AUTH_TOKENS_LOCAL_STORAGE_KEY,
        UseStorageOptions::default().delay_during_hydration(true),
    )
}

const AUTH_TOKENS_LOCAL_STORAGE_KEY: &str = "authTokens";

pub const UNAUTHORIZED_PATHS: &[&str] = &["/sign-up", "/sign-in"];
