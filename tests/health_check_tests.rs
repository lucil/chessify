mod setup;

#[tokio::test]
async fn health_check_works() {
    // Act
    let response = setup::execute_get("/health_check").await;

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
