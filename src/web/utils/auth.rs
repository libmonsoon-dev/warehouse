use crate::dto::AuthTokens;
use leptos::prelude::codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_use::storage::{UseStorageOptions, use_local_storage, use_local_storage_with_options};

#[cfg(feature = "ssr")]
pub fn expect_access_token() -> jsonwebtoken::TokenData<crate::dto::AccessTokenClaims> {
    expect_context()
}

pub fn use_auth_tokens() -> (
    Signal<Option<AuthTokens>>,
    WriteSignal<Option<AuthTokens>>,
    impl Fn() + Clone + Send + Sync,
) {
    use_local_storage::<Option<AuthTokens>, JsonSerdeCodec>(AUTH_TOKENS_LOCAL_STORAGE_KEY)
}

pub fn use_delayed_auth_tokens() -> (
    Signal<Option<AuthTokens>>,
    WriteSignal<Option<AuthTokens>>,
    impl Fn() + Clone + Send + Sync,
) {
    use_local_storage_with_options::<Option<AuthTokens>, JsonSerdeCodec>(
        AUTH_TOKENS_LOCAL_STORAGE_KEY,
        UseStorageOptions::default().delay_during_hydration(true),
    )
}

const AUTH_TOKENS_LOCAL_STORAGE_KEY: &str = "authTokens";

pub const UNAUTHORIZED_PATHS: &[&str] = &["/sign-up", "/sign-in"];
