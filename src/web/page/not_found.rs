use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = crate::web::utils::expect_response_options();
        resp.set_status(http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Page not found"</h1> }
}
