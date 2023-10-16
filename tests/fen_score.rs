use base64::encode;
use chessify::domain::Fen;
mod setup;

#[tokio::test]
async fn fen_score_returns_200() {
    let fen_string = "1r3rk1/p1q2ppp/5b2/8/8/1P2P1P1/P4PKP/3R1R2 w - - 0 22";

    let response = execute_evaluate_score_request(fen_string).await;

    assert!(response.status().is_success());
    let body = response.text().await.unwrap();
    assert_eq!(body, fen_string);
}

#[tokio::test]
async fn fen_score_returns_fen_string() {
    let fen_string = "1r3rk1/p1q2ppp/5b2/8/8/1P2P1P1/P4PKP/3R1R2 w - - 0 22";

    let response = execute_evaluate_score_request(fen_string).await;
    let body = response.text().await.unwrap();
    assert_eq!(body, fen_string);
}

async fn execute_evaluate_score_request(fen_string: &str) -> reqwest::Response {
    let address = setup::spawn_app();
    let client = reqwest::Client::new();
    let fen = Fen::new(fen_string).unwrap();
    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/fen/score/{}", &address, fen.encode()))
        .send()
        .await
        .expect("Failed to execute request.");
    return response;
}
