use crate::web::component::UNAUTHORIZED_PATHS;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

#[component]
pub fn SideBar() -> impl IntoView {
    let location = use_location();
    let display = move || !UNAUTHORIZED_PATHS.contains(&location.pathname.get().as_str());

    view! {
        <Show when=display>
            <aside class="side-bar">
                <nav class="sidebar-nav">
                    <A href="">"Home"</A>
                </nav>
            </aside>
        </Show>
    }
}
