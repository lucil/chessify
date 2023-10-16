use base64::encode;
mod setup;

#[tokio::test]
async fn fen_score_works() {
    let fen_string = "1r3rk1/p1q2ppp/5b2/8/8/1P2P1P1/P4PKP/3R1R2 w - - 0 22";
    // Arrange
    let address = setup::spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/fen/score/{}", &address, encode(fen_string)))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let body = response.text().await.unwrap();
    assert_eq!(body, fen_string);
}
