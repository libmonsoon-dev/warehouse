use crate::helpers::spawn_app;
use chrono::Utc;
use claims::{assert_gt, assert_le};
use fake::Fake;
use pretty_assertions::assert_eq;
use secrecy::ExposeSecret;
use uuid::Uuid;
use warehouse::domain::AuthTokens;
use warehouse::service::auth::AccessTokenClaims;

#[tokio::test]
async fn signup_works() {
    // Arrange
    let app = spawn_app().await;
    let first_name = uuid::fmt::Simple::from_uuid(Uuid::new_v4()).to_string();
    let last_name = uuid::fmt::Simple::from_uuid(Uuid::new_v4()).to_string();
    let password = uuid::fmt::Simple::from_uuid(Uuid::new_v4()).to_string();
    let email = fake::faker::internet::en::SafeEmail().fake::<String>();

    // Act
    let request = serde_json::json!({
        "first_name": &first_name,
        "last_name": &last_name,
        "email": &email,
        "password": &password,
    });

    let response = app
        .sign_up(request.to_string())
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 201);
    let tokens = response
        .json::<AuthTokens>()
        .await
        .expect("Failed to parse response.");

    let access_token =
        jsonwebtoken::dangerous::insecure_decode::<AccessTokenClaims>(tokens.access_token)
            .expect("Failed to decode access token.");

    assert_le!(access_token.claims.iat, Utc::now().timestamp() as usize);
    assert_gt!(access_token.claims.exp, Utc::now().timestamp() as usize);

    let user_repository = app.dependency.user_repository().await;
    let user_in_db = user_repository
        .get_by_id(access_token.claims.id)
        .await
        .expect("Failed to find user by id.");

    assert_eq!(first_name, user_in_db.first_name);
    assert_eq!(last_name, user_in_db.last_name);
    assert_ne!(password, user_in_db.password_hash.expose_secret());
    assert_eq!(email, user_in_db.email);
}

#[tokio::test]
async fn signup_with_existing_email_fails() {
    // Arrange
    let app = spawn_app().await;
    let first_name = uuid::fmt::Simple::from_uuid(Uuid::new_v4()).to_string();
    let last_name = uuid::fmt::Simple::from_uuid(Uuid::new_v4()).to_string();
    let password = uuid::fmt::Simple::from_uuid(Uuid::new_v4()).to_string();
    let email = fake::faker::internet::en::SafeEmail().fake::<String>();

    // Act
    let request = serde_json::json!({
        "first_name": &first_name,
        "last_name": &last_name,
        "email": &email,
        "password": &password,
    });

    let response = app
        .sign_up(request.to_string())
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status(), 201);

    let response = app
        .sign_up(request.to_string())
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 409);
}
