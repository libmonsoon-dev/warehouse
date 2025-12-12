use http::StatusCode;

/// Get health of the API.
#[utoipa::path(
    method(get),
    path = "/health-check",
    responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain")
    )
)]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
