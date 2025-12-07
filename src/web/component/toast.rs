use leptos::children::Children;
use leptos::prelude::*;

#[component]
pub fn ErrorToast(children: Children) -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <div class="toast">
                    <div class="toast-content error-block">
                        <div class="toast-message error-message">
                            <p>"An error occurred:"</p>
                            <ul>
                                {move || {
                                    errors
                                        .get()
                                        .into_iter()
                                        .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                        .collect::<Vec<_>>()
                                }}
                            </ul>
                        </div>
                    </div>
                </div>
            }
        }>{children()}</ErrorBoundary>
    }
}

#[component]
pub fn Toast<W>(children: ChildrenFn, when: W) -> impl IntoView
where
    W: Fn() -> bool + Send + Sync + 'static,
{
    view! {
        <Show when=when>
            <div class="toast">
                <div class="toast-content">
                    <div class="toast-message">{children()}</div>
                </div>
            </div>
        </Show>
    }
}
