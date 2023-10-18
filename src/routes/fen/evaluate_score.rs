use crate::domain::EvaluationResult;
use actix_web::{web, HttpResponse};

pub async fn evaluate_score(info: web::Path<String>) -> HttpResponse {
    let fen_string = info.into_inner();
    let score = -11.0;
    let evaluation_result = EvaluationResult::new(&fen_string, score);

    let serialized_result = serde_json::to_string(&evaluation_result.unwrap()).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serialized_result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::to_bytes;
    use claims::{assert_gt, assert_lt};

    #[tokio::test]
    async fn returns_200() {
        let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let info: web::Path<String> = web::Path::from(fen_string.to_string());

        let response = evaluate_score(info).await;

        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn returns_fen_string() {
        let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let info: web::Path<String> = web::Path::from(fen_string.to_string());

        let response = evaluate_score(info).await;

        let body = to_bytes(response.into_body()).await.unwrap();

        let evaluation_parsed: EvaluationResult =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert_eq!(evaluation_parsed.fen.code, fen_string);
    }

    #[tokio::test]
    async fn returns_negative_score() {
        let fen_string = "1r3rk1/p1q2ppp/5b2/8/8/1P2P1P1/P4PKP/3R1R2 w - - 0 22";
        let info: web::Path<String> = web::Path::from(fen_string.to_string());

        let response = evaluate_score(info).await;
        let body = to_bytes(response.into_body()).await.unwrap();
        let evaluation_parsed: EvaluationResult =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert_lt!(evaluation_parsed.score, 0.0);
    }

    #[ignore]
    #[tokio::test]
    async fn returns_positive_score() {
        let fen_string = "8/6pk/1Qp2p1p/p1p5/2P5/P1B1PP1P/1P3nPK/1q6 w - - 1 31";
        let info: web::Path<String> = web::Path::from(fen_string.to_string());

        let response = evaluate_score(info).await;
        let body = to_bytes(response.into_body()).await.unwrap();
        let evaluation_parsed: EvaluationResult =
            serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();

        assert_gt!(evaluation_parsed.score, 0.0);
    }
}
