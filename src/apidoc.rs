use utoipa::OpenApi;

pub const AUTH_TAG: &str = "Auth";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = AUTH_TAG, description = "Authorization API endpoints"),
    )
)]
pub struct ApiDoc;
