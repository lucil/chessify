use crate::domain::Fen;
use crate::engine::Stockfish;
use crate::evaluator::score_eval::ScoreEval;
use crate::evaluator::score_eval::ScoreEvaluator;
use actix_web::{web, HttpResponse};

pub async fn fen_score(fen_string: web::Path<String>) -> HttpResponse {
    let fen_string = fen_string.into_inner();

    let score_evaluator = ScoreEval::new();
    let evaluation_result =
        score_evaluator.evaluate_score(&Stockfish::new(), Fen::new(&fen_string).unwrap());
    let serialized_result = serde_json::to_string(&evaluation_result.unwrap()).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serialized_result)
}
