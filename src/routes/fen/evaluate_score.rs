use actix_web::{web, HttpResponse};

use crate::domain::Fen;

pub async fn evaluate_score(info: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(Fen::decode(info.into_inner()))
}
