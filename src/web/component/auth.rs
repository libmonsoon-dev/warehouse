use crate::domain::AuthTokens;
use codee::string::JsonSerdeCodec;
use leptos::component;
use leptos::prelude::*;
use leptos::{IntoView, view};
use leptos_use::storage::use_local_storage;

#[component]
pub fn Authorized(children: Children) -> impl IntoView {
    let (tokens, _, _) = use_auth_tokens();
    let navigate = leptos_router::hooks::use_navigate();

    Effect::new(move |_| {
        if let None = tokens.get() {
            navigate("/sign-in", Default::default());
        }
    });

    view! { {children()} }
}

pub fn use_auth_tokens() -> (
    Signal<Option<AuthTokens>>,
    WriteSignal<Option<AuthTokens>>,
    impl Fn() + Clone + Send + Sync,
) {
    use_local_storage::<Option<AuthTokens>, JsonSerdeCodec>("authTokens")
}
