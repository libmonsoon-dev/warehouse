use crate::helpers::spawn_app;
use chrono::Utc;
use claims::{assert_gt, assert_le};
use fake::Fake;
use pretty_assertions::assert_eq;
use secrecy::ExposeSecret;
use uuid::Uuid;
use warehouse::contract::error::ErrorCode;
use warehouse::dto::{AccessTokenClaims, AppError, AuthTokens};

#[tokio::test]
async fn sign_in_works() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let request = serde_json::json!({
        "email": &app.data.admin.email,
        "password": &app.data.admin.password.expose_secret(),
    });

    let response = app
        .sign_in(request.to_string())
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 200);
    let tokens = response
        .json::<AuthTokens>()
        .await
        .expect("Failed to parse response.");

    let access_token =
        jsonwebtoken::dangerous::insecure_decode::<AccessTokenClaims>(tokens.access_token)
            .expect("Failed to decode access token.");

    assert_le!(access_token.claims.iat, Utc::now().timestamp());
    assert_gt!(access_token.claims.exp, Utc::now().timestamp());
}

#[tokio::test]
async fn sign_in_with_invalid_email_fails() {
    // Arrange
    let app = spawn_app().await;
    let email = fake::faker::internet::en::SafeEmail().fake::<String>();
    let password = app.data.admin.password.expose_secret();

    // Act
    let request = serde_json::json!({
        "email": &email,
        "password": &password,
    });

    let response = app
        .sign_in(request.to_string())
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 401);
    assert_eq!(
        response.json::<AppError>().await.unwrap(),
        AppError {
            code: ErrorCode::AuthenticationFailed,
            message: "Invalid login or password".to_string()
        }
    );
}

#[tokio::test]
async fn sign_in_with_invalid_password_fails() {
    // Arrange
    let app = spawn_app().await;
    let email = app.data.admin.email.clone();
    let password = uuid::fmt::Simple::from_uuid(Uuid::new_v4()).to_string();

    // Act
    let request = serde_json::json!({
        "email": &email,
        "password": &password,
    });

    let response = app
        .sign_in(request.to_string())
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 401);
    assert_eq!(
        response.json::<AppError>().await.unwrap(),
        AppError {
            code: ErrorCode::AuthenticationFailed,
            message: "Invalid login or password".to_string()
        }
    );
}
