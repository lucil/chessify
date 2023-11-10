use crate::domain::{EvaluationResult, Fen};
use crate::engine::{commands, parse_go_depth_result, Engine, Stockfish};
use actix_web::{web, HttpResponse};

pub async fn fen_score(fen_string: web::Path<String>) -> HttpResponse {
    let fen_string = fen_string.into_inner();

    let evaluation_result = evaluate_score(&Stockfish::new(), Fen::new(&fen_string).unwrap());
    let serialized_result = serde_json::to_string(&evaluation_result.unwrap()).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serialized_result)
}

fn evaluate_score(engine: &impl Engine, fen: Fen) -> Result<EvaluationResult, String> {
    let execution_result =
        engine.execute(vec![commands::position_fen(&fen), commands::go_depth(10)]);

    EvaluationResult::build(fen, parse_go_depth_result(&execution_result))
}

#[cfg(test)]
mod evaluate_score_tests {
    use super::*;
    use crate::domain::{Fen, Score};
    use crate::engine::MockStockfish;

    #[test]
    fn evaluate_score_returns_2() {
        let mut mocked_stockfish = MockStockfish::new();
        mocked_stockfish
            .expect_execute()
            .return_const(score_cp(200));
        let evaluation = evaluate_score(&mocked_stockfish, Fen::new("some fen").unwrap());
        assert_eq!(evaluation.unwrap().score, Score::new(2.0))
    }

    #[test]
    fn evaluate_score_parses_return_result() {
        let mut mocked_stockfish = MockStockfish::new();
        mocked_stockfish.expect_execute().return_const(score_cp(50));

        let evaluation = evaluate_score(&mocked_stockfish, Fen::new("some fen").unwrap());
        assert_eq!(evaluation.unwrap().score, Score::new(0.5))
    }

    fn score_cp(score: i32) -> String {
        format!("info depth 10 seldepth 10 multipv 1 score cp {} nodes 7548 nps 471750 hashfull 2 tbhits 0 time 16 pv e2e4 e7e5 g1f3 g8f6 b1c3 d7d6 d2d4\nbestmove e2e4 ponder e7e5\n", score)
    }
}
