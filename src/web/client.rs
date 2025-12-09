use crate::contract::http::{AUTHORIZATION_HEADER, AUTHORIZATION_SCHEME};
use crate::web::component::use_auth_tokens;
use futures::{Sink, Stream};
use leptos::server_fn::response::ClientRes;
use leptos::{
    prelude::*, server_fn::client::Client, server_fn::client::browser::BrowserClient,
    server_fn::request::browser::BrowserRequest, server_fn::response::browser::BrowserResponse,
};

pub struct CustomClient;

impl<E, IS, OS> Client<E, IS, OS> for CustomClient
where
    E: FromServerFnError,
    IS: FromServerFnError,
    OS: FromServerFnError,
{
    type Request = BrowserRequest;
    type Response = BrowserResponse;

    async fn send(req: Self::Request) -> Result<Self::Response, E> {
        let (tokens, set_tokens, _) = use_auth_tokens();

        let Some(tokens) = tokens.get() else {
            return <BrowserClient as Client<E, IS, OS>>::send(req).await;
        };

        //TODO: refresh token
        req.headers().append(
            AUTHORIZATION_HEADER,
            &format!("{} {}", AUTHORIZATION_SCHEME, tokens.access_token),
        );

        let response = <BrowserClient as Client<E, IS, OS>>::send(req).await?;

        if <BrowserResponse as ClientRes<E>>::status(&response)
            == http::StatusCode::UNAUTHORIZED.as_u16()
        {
            set_tokens.set(None)
        }

        Ok(response)
    }

    fn open_websocket(
        path: &str,
    ) -> impl Future<
        Output = Result<
            (
                impl Stream<Item = Result<server_fn::Bytes, server_fn::Bytes>> + Send + 'static,
                impl Sink<server_fn::Bytes> + Send + 'static,
            ),
            E,
        >,
    > + Send {
        <BrowserClient as Client<E, IS, OS>>::open_websocket(path)
    }

    fn spawn(future: impl Future<Output = ()> + Send + 'static) {
        <BrowserClient as Client<E, IS, OS>>::spawn(future)
    }
}
