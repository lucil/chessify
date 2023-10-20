use crate::{domain::EvaluationResult, evaluator::Engine};
use actix_web::{web, HttpResponse};

pub async fn evaluate_fen(info: web::Path<String>) -> HttpResponse {
    let fen_string = info.into_inner();

    let engine = crate::evaluator::StockfishEngine::new();
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
