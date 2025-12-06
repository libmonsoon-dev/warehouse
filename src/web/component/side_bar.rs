use leptos::prelude::*;
use leptos::{IntoView, component, view};
use leptos_router::components::A;

#[component]
pub fn SideBar() -> impl IntoView {
    view! {
          <aside class="side-bar">
        <nav class="sidebar-nav">
            <A href="">"Home"</A>
        </nav>
        </aside>
    }
}
