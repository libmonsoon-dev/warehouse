use crate::web::utils::{use_auth_tokens, use_delayed_auth_tokens};
use leptos::prelude::*;

#[component]
pub fn Authorized(children: Children) -> impl IntoView {
    let (tokens, _, _) = use_auth_tokens();
    let navigate = leptos_router::hooks::use_navigate();

    Effect::new(move |_| {
        if tokens.get().is_none() {
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
