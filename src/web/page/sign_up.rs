use crate::dto::{AuthTokens, SignUpRequest};
use crate::web::component::{use_auth_tokens, ErrorToast, Toast, WebError};
use leptos::prelude::*;
use leptos_router::components::A;
use validator::Validate;

#[component]
pub fn SignUp() -> impl IntoView {
    let (tokens, set_tokens, _) = use_auth_tokens();
    let navigate = leptos_router::hooks::use_navigate();

    Effect::new(move |_| {
        if let Some(_) = tokens.get() {
            navigate("/", Default::default());
        }
    });

    let (first_name, set_first_name) = signal(String::new());
    let (last_name, set_last_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());

    let req = move || SignUpRequest {
        first_name: first_name.get(),
        last_name: last_name.get(),
        email: email.get(),
        password: password.get(),
    };

    let sign_up_action = Action::<SignUpRequest, Result<(), WebError>>::new(move |input| {
        let input = input.to_owned();
        async move {
            //TODO: reactive validation
            input.validate()?;
            let tokens = sign_up(input).await?;
            set_tokens.set(Some(tokens));
            Ok(())
        }
    });

    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            sign_up_action.dispatch(req());
        }>
            <input
                type="text"
                placeholder="First Name"
                on:input:target=move |ev| set_first_name.set(ev.target().value())
                prop:value=first_name
            />
            <input
                type="text"
                placeholder="Second Name"
                on:input:target=move |ev| set_last_name.set(ev.target().value())
                prop:value=last_name
            />
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
        <p>"Already have an account? "<A href="/sign-in">Sign in</A></p>
        <Toast when=move || sign_up_action.pending().get()>
            <p>"Signing in..."</p>
        </Toast>
        <ErrorToast>{move || sign_up_action.value().get()}</ErrorToast>
    }
}

#[server]
//TODO: #[tracing::instrument(skip(req))]
async fn sign_up(req: SignUpRequest) -> Result<AuthTokens, ServerFnError> {
    use crate::state::AppState;

    //TODO: error type
    //TODO: error message
    req.validate()?;

    let tokens = expect_context::<AppState>()
        .dependencies
        .auth_service()
        .await
        .sign_up(req.into())
        .await
        .map_err(ServerFnError::new)?;

    Ok(tokens)
}
