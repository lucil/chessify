#[cfg(test)]
mod evaluate_score_test {
    use crate::{
        domain::EvaluationResult,
        evaluator::MockEngine,
        routes::fen::evaluate_score::{self, engine_evaluation},
    };
    use actix_web::{body::to_bytes, web};
    use claims::{assert_gt, assert_lt};
    use evaluate_score::evaluate_fen;

    async fn evaluate_and_parse(fen_string: &str) -> EvaluationResult {
        let info: web::Path<String> = web::Path::from(fen_string.to_string());

        let response = evaluate_fen(info).await;
        let body = to_bytes(response.into_body()).await.unwrap();

        serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap()
    }

    #[tokio::test]
    async fn returns_200() {
        let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let info: web::Path<String> = web::Path::from(fen_string.to_string());

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

        let mut engine = MockEngine::new();
        engine
            .expect_evaluate_fen()
            .withf(move |fen| fen == fen_string)
            .returning(|_| -1.0);

        let result = engine_evaluation(fen_string, &engine).unwrap();

        assert_lt!(result.score, 0.0);
    }

    #[test]
    fn returns_positive_score() {
        let fen_string = "8/6pk/1Qp2p1p/p1p5/2P5/P1B1PP1P/1P3nPK/1q6 w - - 1 31";

        let mut engine = MockEngine::new();
        engine
            .expect_evaluate_fen()
            .withf(move |fen| fen == fen_string)
            .returning(|_| 1.0);

        let result = engine_evaluation(fen_string, &engine).unwrap();

        assert_gt!(result.score, 0.0);
    }
}
