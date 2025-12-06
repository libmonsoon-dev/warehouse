use crate::web::component::LogOutButton;
use leptos::prelude::*;
use leptos::{IntoView, component, view};

#[component]
pub fn TopBar() -> impl IntoView {
    view! {
        <header class="top-bar">
            <LogOutButton />
        </header>
    }
}
