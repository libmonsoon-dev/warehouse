use crate::dto::AccessTokenClaims;
use crate::web::client::CustomClient;
use crate::web::component::Authorized;
use leptos::prelude::*;
use tracing_log::log;
use crate::web::error::ServerError;

#[component]
pub fn HomePage() -> impl IntoView {
    //TODO: remove this
    let decode_jwt_resource = LocalResource::new(move || decode_jwt());

    Effect::new(move |_| {
        log::info!("auth token {:?}", decode_jwt_resource.get());
    });

    view! {
        <Authorized>
            <h1>"Welcome to warehouse!"</h1>
        </Authorized>
    }
}

#[server(client=CustomClient)]
#[middleware(crate::web::middleware::AuthorizationLayer)]
async fn decode_jwt() -> Result<AccessTokenClaims, ServerError> {
    use crate::web::utils::expect_access_token;

    Ok(expect_access_token().claims)
}
