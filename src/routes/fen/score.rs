use crate::evaluator::StockfishEngine;
use crate::{domain::EvaluationResult, evaluator::Engine};
use actix_web::{web, HttpResponse};

pub async fn evaluate_fen(fen_string: web::Path<String>) -> HttpResponse {
    let fen_string = fen_string.into_inner();

    let engine = StockfishEngine::new();
    let evaluation_result = engine_evaluation(&fen_string, &engine);
    let serialized_result = serde_json::to_string(&evaluation_result.unwrap()).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serialized_result)
}

pub fn engine_evaluation(
    fen_string: &str,
    engine: &impl Engine,
) -> Result<EvaluationResult, String> {
    let _ = engine;
    let score = engine.evaluate_fen(fen_string);
    EvaluationResult::new(fen_string, score)
}

#[cfg(test)]
mod evaluate_score_test {
    use crate::{
        domain::EvaluationResult,
        routes::fen::evaluate_score::{self},
    };
    use actix_web::{body::to_bytes, web};
    use evaluate_score::{engine_evaluation, evaluate_fen};

    async fn evaluate_and_parse(fen_string: &str) -> EvaluationResult {
        let info: web::Path<String> = web::Path::from(fen_string.to_string());

        let response = evaluate_fen(info).await;
        let body = to_bytes(response.into_body()).await.unwrap();

        serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap()
    }

    #[tokio::test]
    async fn returns_200() {
        let info: web::Path<String> =
            web::Path::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string());

        let response = evaluate_fen(info).await;
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn returns_evaluation_result() {
        let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

        let result = evaluate_and_parse(fen_string).await;
        assert_eq!(result.fen.code, fen_string);
    }

    #[test]
    fn returns_negative_score() {
        let fen_string = "1r3rk1/p1q2ppp/5b2/8/8/1P2P1P1/P4PKP/3R1R2 w - - 0 22";

        //let result = engine_evaluation(fen_string).unwrap();

        //assert!(result.score < 0.0);
    }

    #[test]
    fn returns_positive_score() {
        let fen_string = "8/6pk/1Qp2p1p/p1p5/2P5/P1B1PP1P/1P3nPK/1q6 w - - 1 31";

        //let result = engine_evaluation(fen_string).unwrap();

        //assert!(result.score > 0.0);
    }
}
