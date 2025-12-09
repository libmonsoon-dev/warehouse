use crate::web::component::LogOutButton;
use crate::web::utils::UNAUTHORIZED_PATHS;
use leptos::prelude::*;
use leptos_router::hooks::use_location;

#[component]
pub fn TopBar() -> impl IntoView {
    let location = use_location();
    let display = move || !UNAUTHORIZED_PATHS.contains(&location.pathname.get().as_str());

    view! {
        <Show when=display>
            <header class="top-bar">
                <LogOutButton />
            </header>
        </Show>
    }
}
