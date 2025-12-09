use crate::dto::AccessTokenClaims;
use crate::web::client::CustomClient;
use crate::web::component::Authorized;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let decode_jwt_resource = LocalResource::new(move || decode_jwt());

    view! {
        <Authorized>
            <h1>"Welcome to warehouse!"</h1>
            <code>{move || serde_json::to_string_pretty(&decode_jwt_resource.get())}</code>
        </Authorized>
    }
}

#[server(client=CustomClient)]
#[middleware(crate::web::middleware::AuthorizationLayer)]
async fn decode_jwt() -> Result<AccessTokenClaims, ServerFnError> {
    use crate::web::utils::expect_access_token;

    Ok(expect_access_token().claims)
}
