use std::collections::HashMap;

use chessify::domain::Fen;
use claims::assert_gt;
use claims::assert_lt;
use serde_json::Value;
mod setup;

#[tokio::test]
async fn fen_score_returns_200() {
    let fen_string = "1r3rk1/p1q2ppp/5b2/8/8/1P2P1P1/P4PKP/3R1R2 w - - 0 22";

    let response = execute_evaluate_score_request(fen_string).await;

    assert!(response.status().is_success());
}

#[tokio::test]
async fn fen_score_returns_bad_request() {
    let fen_string = "";

    let response = execute_evaluate_score_request(fen_string).await;

    assert!(response.status().is_client_error());
}

#[tokio::test]
async fn fen_score_returns_negative_score() {
    let fen_string = "1r3rk1/p1q2ppp/5b2/8/8/1P2P1P1/P4PKP/3R1R2 w - - 0 22";
    let response = execute_evaluate_score_request(fen_string).await;
    let body = response.text().await.unwrap();

    let evaluation_parsed: HashMap<String, Value> = serde_json::from_str(&body).unwrap();
    assert_eq!(
        evaluation_parsed.get("fen").unwrap().get("code").unwrap(),
        fen_string
    );

    assert_lt!(
        evaluation_parsed.get("score").unwrap().as_f64().unwrap(),
        0.0
    );
}

#[tokio::test]
async fn fen_score_returns_positive_score() {
    let fen_string = "5rk1/p4ppp/8/8/8/1P2P1P1/P4PKP/3R1R2 w - - 0 22";
    let response = execute_evaluate_score_request(fen_string).await;
    let body = response.text().await.unwrap();

    let evaluation_parsed: HashMap<String, Value> = serde_json::from_str(&body).unwrap();
    assert_eq!(
        evaluation_parsed.get("fen").unwrap().get("code").unwrap(),
        fen_string
    );

    assert_gt!(
        evaluation_parsed.get("score").unwrap().as_f64().unwrap(),
        0.0
    );
}

async fn execute_evaluate_score_request(fen_string: &str) -> reqwest::Response {
    let address = setup::spawn_app();
    let client = reqwest::Client::new();
    let mut fen_parameter = String::from("");
    if !fen_string.is_empty() {
        fen_parameter = Fen::new(fen_string).unwrap().encode();
    }

    client
        // Use the returned application address
        .get(&format!("{}/fen/score/{}", &address, fen_parameter))
        .send()
        .await
        .expect("Failed to execute request.")
}
