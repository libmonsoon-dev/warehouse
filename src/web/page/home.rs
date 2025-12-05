use crate::web::component::auth::Authorized;
use leptos::prelude::*;
use leptos::{IntoView, component, view};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Authorized>
            <h1>"Welcome to warehouse!"</h1>
        </Authorized>
    }
}
