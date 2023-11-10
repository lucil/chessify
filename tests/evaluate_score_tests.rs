use serde_json::Value;
use std::collections::HashMap;
mod setup;

const VALID_FEN_POSITIVE: &str = "8/6pk/1Qp2p1p/p1p5/2P5/P1B1PP1P/1P3nPK/1q6 w - - 1 31";
const VALID_FEN_NEGATIVE: &str = "1r3rk1/p1q2ppp/5b2/8/8/1P2P1P1/P4PKP/3R1R2 w - - 0 22";

async fn execute_evaluate_score_request(fen_string: &str) -> reqwest::Response {
    let address = setup::spawn_app();
    let client = reqwest::Client::new();
    let fen_parameter = urlencoding::encode(fen_string);

    client
        .get(&format!("{}/fen/score/{}", &address, fen_parameter))
        .send()
        .await
        .expect("Failed to execute request.")
}

fn build_url(fen_string: &str) -> String {
    let fen_parameter = urlencoding::encode(fen_string);
    format!("/fen/score/{}", fen_parameter)
}

#[tokio::test]
async fn evaluate_score_returns_success_with_valid_fen() {
    let response = setup::execute_get(&build_url(VALID_FEN_NEGATIVE)).await;

    assert!(response.status().is_success());
}

#[tokio::test]
async fn evaluate_score_returns_bad_request_on_empty_fen() {
    let response = setup::execute_get(&build_url("")).await;

    assert!(response.status().is_client_error());
}

#[tokio::test]
async fn evaluate_score_returns_fen_and_negative_score() {
    evaluate_fen_and_score(VALID_FEN_NEGATIVE, -15.0, -5.0).await;
}

#[tokio::test]
async fn evaluate_score_returns_fen_and_positive_score() {
    evaluate_fen_and_score(VALID_FEN_POSITIVE, 1.0, 5.0).await;
}

async fn evaluate_fen_and_score(fen_string: &str, min_score: f64, max_score: f64) {
    let response = setup::execute_get(&build_url(fen_string)).await;
    let body = response.text().await.unwrap();

    let evaluation_parsed: HashMap<String, Value> = serde_json::from_str(&body).unwrap();
    assert_eq!(
        evaluation_parsed.get("fen").unwrap().get("code").unwrap(),
        fen_string
    );

    let score = evaluation_parsed
        .get("score")
        .unwrap()
        .get("value")
        .unwrap()
        .as_f64()
        .unwrap();
    assert!(score > min_score && score < max_score);
}
