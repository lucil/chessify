use actix_web::{web, HttpResponse};
use base64::decode;

pub async fn evaluate_score(info: web::Path<String>) -> HttpResponse {
    let fen_string = info.into_inner();
    let decoded = decode(fen_string).unwrap();
    let decoded_str = String::from_utf8(decoded).unwrap();
    HttpResponse::Ok().body(decoded_str)
}
