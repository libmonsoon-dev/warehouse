use crate::domain::AuthTokens;
use crate::dto::auth::SignInRequest;
use crate::web::component::{ErrorMessage, WebError, use_auth_tokens};
use leptos::prelude::*;
use leptos::{IntoView, component, view};
use leptos_router::components::A;
use validator::Validate;

#[component]
pub fn SignIn() -> impl IntoView {
    let (tokens, set_tokens, _) = use_auth_tokens();
    let navigate = leptos_router::hooks::use_navigate();

    Effect::new(move |_| {
        if let Some(_) = tokens.get() {
            navigate("/", Default::default());
        }
    });

    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());

    let req = move || SignInRequest {
        email: email.get(),
        password: password.get(),
    };

    let sign_in_action = Action::<SignInRequest, Result<(), WebError>>::new(move |input| {
        let input = input.to_owned();
        async move {
            //TODO: reactive validation
            input.validate()?;
            let tokens = sign_in(input).await?;
            set_tokens.set(Some(tokens));
            Ok(())
        }
    });

    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            sign_in_action.dispatch(req());
        }>
            <input
                type="email"
                placeholder="Email Address"
                on:input:target=move |ev| set_email.set(ev.target().value())
                prop:value=email
            />
            <input
                type="password"
                placeholder="Password"
                on:input:target=move |ev| set_password.set(ev.target().value())
                prop:value=password
            />

            <button type="submit">Sign up</button>
        </form>
        {move || sign_in_action.pending().get().then_some(view! { <p>"Signing in..."</p> })}
        <p>"Don't have an account yet? "<A href="/sign-up">"Sign up"</A></p>
        <ErrorMessage>{move || sign_in_action.value().get()}</ErrorMessage>
    }
}

#[server]
//TODO: #[tracing::instrument(skip(req))]
async fn sign_in(req: SignInRequest) -> Result<AuthTokens, ServerFnError> {
    use crate::state::AppState;

    //TODO: error type
    req.validate()?;

    let tokens = expect_context::<AppState>()
        .dependencies
        .auth_service()
        .await
        .sign_in(req.into())
        .await
        .map_err(ServerFnError::new)?;

    Ok(tokens)
}
