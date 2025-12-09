use crate::contract::http::{AUTHORIZATION_HEADER, AUTHORIZATION_SCHEME};
use crate::state::AppState;
use anyhow::{Context, Error, anyhow};
use axum::body::Body;
use http::Request;
use http::StatusCode;
use http::header::HeaderMap;
use leptos::prelude::*;
use leptos::server_fn::middleware::BoxedService;
use leptos_axum::ResponseOptions;
use leptos_axum::extract;
use std::{pin::Pin, task::Poll};
use tower::{Layer, Service};

pub struct AuthorizationLayer;

impl Layer<BoxedService<http::Request<axum::body::Body>, http::Response<axum::body::Body>>>
    for AuthorizationLayer
{
    type Service = AuthorizationService;

    fn layer(
        &self,
        inner: BoxedService<http::Request<axum::body::Body>, http::Response<axum::body::Body>>,
    ) -> Self::Service {
        AuthorizationService { inner }
    }
}

pub struct AuthorizationService {
    inner: BoxedService<Request<Body>, http::Response<Body>>,
}

impl Service<Request<Body>> for AuthorizationService {
    type Response = http::Response<Body>;
    type Error = ServerFnError;
    type Future =
        Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let future = self.inner.call(req);

        Box::pin(async move {
            let response = expect_context::<ResponseOptions>();
            println!("Response: {response:?}");
            match provide_access_token().await {
                Ok(_) => {}
                Err(err) => {
                    tracing::error!("authorization middleware: {:?}", err);
                    response.set_status(StatusCode::UNAUTHORIZED);
                    return Err(ServerFnError::MiddlewareError("Unauthorized".to_owned()));
                }
            };

            future.await
        })
    }
}

async fn provide_access_token() -> Result<(), Error> {
    let headers: HeaderMap = extract().await?;
    let header = headers.get(AUTHORIZATION_HEADER);
    let Some(header) = header else {
        return Err(anyhow!("Missing {AUTHORIZATION_HEADER} header"));
    };

    let Ok(header) = header.to_str() else {
        return Err(anyhow!(
            "{AUTHORIZATION_HEADER} header must contain visible ASCII chars"
        ));
    };

    let Some(token) = header.strip_prefix(&format!("{AUTHORIZATION_SCHEME} ")) else {
        return Err(anyhow!(
            "{AUTHORIZATION_HEADER} contains an unsupported authorization scheme"
        ));
    };

    let token = expect_context::<AppState>()
        .dependencies
        .auth_service()
        .await
        .decode_access_jwt(token)
        .context("decode access jwt")?;

    provide_context(token);
    Ok(())
}
