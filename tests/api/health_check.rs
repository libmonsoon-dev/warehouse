use crate::helpers::spawn_app;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app
        .health_check()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 200);
    assert_eq!(response.content_length(), Some(0));
}
