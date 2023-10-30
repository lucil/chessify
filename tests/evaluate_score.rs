use chessify::domain::Fen;
use serde_json::Value;
use std::collections::HashMap;
mod setup;

const VALID_FEN_NEGATIVE: &str = "1r3rk1/p1q2ppp/5b2/8/8/1P2P1P1/P4PKP/3R1R2 w - - 0 22";
const VALID_FEN_POSITIVE: &str = "8/6pk/1Qp2p1p/p1p5/2P5/P1B1PP1P/1P3nPK/1q6 w - - 1 31";

#[tokio::test]
async fn returns_200() {
    let response = execute_evaluate_score_request(VALID_FEN_NEGATIVE).await;

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn evaluate_score_returns_success_with_valid_fen() {
    let fen_string = VALID_FEN_NEGATIVE;

    let response = execute_evaluate_score_request(fen_string).await;

    assert!(response.status().is_success());
}

#[tokio::test]
async fn evaluate_score_returns_bad_request_on_empty_fen() {
    let fen_string = "";

    let response = execute_evaluate_score_request(fen_string).await;

    assert!(response.status().is_client_error());
}

#[tokio::test]
async fn evaluate_score_returns_fen_and_negative_score() {
    let response = execute_evaluate_score_request(VALID_FEN_NEGATIVE).await;
    let body = response.text().await.unwrap();

    let evaluation_parsed: HashMap<String, Value> = serde_json::from_str(&body).unwrap();
    assert_eq!(
        evaluation_parsed.get("fen").unwrap().get("code").unwrap(),
        VALID_FEN_NEGATIVE
    );

    let score = evaluation_parsed.get("score").unwrap().as_f64().unwrap();

    assert!(score < -1.0);
    assert!(score > -10.0);
}

#[tokio::test]
async fn evaluate_score_returns_fen_and_positive_score() {
    let response = execute_evaluate_score_request(VALID_FEN_POSITIVE).await;
    let body = response.text().await.unwrap();

    let evaluation_parsed: HashMap<String, Value> = serde_json::from_str(&body).unwrap();
    assert_eq!(
        evaluation_parsed.get("fen").unwrap().get("code").unwrap(),
        VALID_FEN_POSITIVE
    );
    let score = evaluation_parsed.get("score").unwrap().as_f64().unwrap();
    assert!(score > 1.0);
    assert!(score < 2.0);
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
