use crate::domain::EvaluationResult;
use actix_web::{web, HttpResponse};

pub async fn evaluate_score(info: web::Path<String>) -> HttpResponse {
    let fen_string = info.into_inner();
    let evaluation_result = EvaluationResult::new(&fen_string, -11.0);

    let serialized_result = serde_json::to_string(&evaluation_result.unwrap())
        .expect("Failed to serialize evaluation result to JSON");

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serialized_result)
}
