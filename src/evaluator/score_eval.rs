use crate::domain::{EvaluationResult, Fen};
use crate::engine::{commands, parse_go_depth_result, Engine};

pub trait ScoreEvaluator {
    fn evaluate_score(&self, engine: &impl Engine, fen: Fen) -> Result<EvaluationResult, String>;
}

#[derive(Debug, Default)]
pub struct ScoreEval {}

impl ScoreEvaluator for ScoreEval {
    fn evaluate_score(&self, engine: &impl Engine, fen: Fen) -> Result<EvaluationResult, String> {
        let execution_result =
            engine.execute(vec![commands::position_fen(&fen), commands::go_depth(10)]);

        EvaluationResult::build(fen, parse_go_depth_result(&execution_result))
    }
}

impl ScoreEval {
    pub fn new() -> ScoreEval {
        ScoreEval {}
    }
}

#[cfg(test)]
mod score_eval_tests {
    use super::*;
    use crate::domain::{Fen, Score};
    use crate::engine::MockStockfish;

    #[test]
    fn evaluate_score_returns_2() {
        let mut mocked_stockfish = MockStockfish::new();
        mocked_stockfish
            .expect_execute()
            .return_const(score_cp(200));

        let score_eval = ScoreEval::new();
        let evaluation =
            score_eval.evaluate_score(&mocked_stockfish, Fen::new("some fen").unwrap());
        assert_eq!(evaluation.unwrap().score, Score::new(2.0))
    }

    #[test]
    fn evaluate_score_parses_return_result() {
        let mut mocked_stockfish = MockStockfish::new();
        mocked_stockfish.expect_execute().return_const(score_cp(50));

        let score_eval = ScoreEval::new();
        let evaluation =
            score_eval.evaluate_score(&mocked_stockfish, Fen::new("some fen").unwrap());
        assert_eq!(evaluation.unwrap().score, Score::new(0.5))
    }

    fn score_cp(score: i32) -> String {
        format!("info depth 10 seldepth 10 multipv 1 score cp {} nodes 7548 nps 471750 hashfull 2 tbhits 0 time 16 pv e2e4 e7e5 g1f3 g8f6 b1c3 d7d6 d2d4\nbestmove e2e4 ponder e7e5\n", score)
    }
}
