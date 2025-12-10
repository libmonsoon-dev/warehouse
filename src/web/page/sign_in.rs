use crate::dto::{AuthTokens, SignInRequest};
use crate::web::component::{ErrorToast, Toast, WebError};
use crate::web::utils::use_auth_tokens;
use leptos::prelude::*;
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
        <p>"Don't have an account yet? "<A href="/sign-up">"Sign up"</A></p>
        <Toast when=move || sign_in_action.pending().get()>
            <p>"Signing in..."</p>
        </Toast>
        <ErrorToast>{move || sign_in_action.value().get()}</ErrorToast>
    }
}

#[tracing::instrument(skip(req))]
#[server]
async fn sign_in(req: SignInRequest) -> Result<AuthTokens, ServerFnError> {
    //TODO: error type
    //TODO: error message
    req.validate()?;

    let tokens = crate::web::utils::expect_app_state()
        .dependencies
        .auth_service()
        .await
        .sign_in(req.into())
        .await
        .map_err(ServerFnError::new)?;

    Ok(tokens)
}
