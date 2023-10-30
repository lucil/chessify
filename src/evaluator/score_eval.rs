use crate::domain::{EvaluationResult, Fen};
use crate::engine::{commands, Engine};

pub trait ScoreEvaluator {
    fn evaluate_score(&self, engine: &impl Engine, fen: Fen) -> Result<EvaluationResult, String>;
}

#[derive(Debug, Default)]
pub struct ScoreEval {}

impl ScoreEvaluator for ScoreEval {
    fn evaluate_score(&self, engine: &impl Engine, fen: Fen) -> Result<EvaluationResult, String> {
        let execution_result =
            engine.execute(vec![commands::position_fen(&fen), commands::go_depth(10)]);

        let mut last_cp_score: Option<f32> = None;

        for line in execution_result.lines() {
            if let Some(score) = line.strip_prefix("info depth") {
                let score_parts: Vec<&str> = score.split("score cp ").collect();
                if score_parts.len() > 1 {
                    if let Ok(cp) = score_parts[1]
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .parse::<f32>()
                    {
                        last_cp_score = Some(cp);
                    }
                }
            }
        }

        EvaluationResult::from_fen(fen, last_cp_score.unwrap() / 100.0)
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
    use crate::domain::Fen;
    use crate::engine::MockStockfish;

    #[test]
    fn evaluate_score_returns_2() {
        let mut mocked_stockfish = MockStockfish::new();
        mocked_stockfish.expect_execute().return_const(score200cp());

        let score_eval = ScoreEval::new();
        let evaluation =
            score_eval.evaluate_score(&mocked_stockfish, Fen::new("some fen").unwrap());
        assert_eq!(evaluation.unwrap().score, 2.0)
    }

    #[test]
    fn evaluate_score_parses_return_result() {
        let mut mocked_stockfish = MockStockfish::new();
        mocked_stockfish.expect_execute().return_const(score50cp());

        let score_eval = ScoreEval::new();
        let evaluation =
            score_eval.evaluate_score(&mocked_stockfish, Fen::new("some fen").unwrap());
        assert_eq!(evaluation.unwrap().score, 0.5)
    }

    fn score50cp() -> String {
        "Stockfish 16 by the Stockfish developers (see AUTHORS file)\ninfo string NNUE evaluation using nn-5af11540bbfe.nnue enabled\ninfo depth 1 seldepth 1 multipv 1 score cp 2 nodes 20 nps 20000 hashfull 0 tbhits 0 time 1 pv g1f3\ninfo depth 2 seldepth 2 multipv 1 score cp 2 nodes 40 nps 40000 hashfull 0 tbhits 0 time 1 pv g1f3\ninfo depth 3 seldepth 2 multipv 1 score cp 16 nodes 70 nps 70000 hashfull 0 tbhits 0 time 1 pv c2c3\ninfo depth 4 seldepth 2 multipv 1 score cp 29 nodes 101 nps 101000 hashfull 0 tbhits 0 time 1 pv e2e4\ninfo depth 5 seldepth 3 multipv 1 score cp 42 nodes 131 nps 131000 hashfull 0 tbhits 0 time 1 pv e2e4 g8f6\ninfo depth 6 seldepth 4 multipv 1 score cp 59 nodes 489 nps 244500 hashfull 0 tbhits 0 time 2 pv g1f3 d7d5 d2d4\ninfo depth 7 seldepth 6 multipv 1 score cp 31 nodes 1560 nps 390000 hashfull 1 tbhits 0 time 4 pv e2e4 d7d5 e4d5 d8d5 g1f3\ninfo depth 8 seldepth 6 multipv 1 score cp 40 nodes 2105 nps 421000 hashfull 1 tbhits 0 time 5 pv e2e4 d7d5 e4d5 d8d5\ninfo depth 9 seldepth 8 multipv 1 score cp 48 nodes 4500 nps 450000 hashfull 1 tbhits 0 time 10 pv e2e4 e7e5 g1f3 g8f6 f3e5 f6e4 d2d4 b8c6\ninfo depth 10 seldepth 10 multipv 1 score cp 50 nodes 7548 nps 471750 hashfull 2 tbhits 0 time 16 pv e2e4 e7e5 g1f3 g8f6 b1c3 d7d6 d2d4\nbestmove e2e4 ponder e7e5\n".to_string()
    }

    fn score200cp() -> String {
        "Stockfish 16 by the Stockfish developers (see AUTHORS file)\ninfo string NNUE evaluation using nn-5af11540bbfe.nnue enabled\ninfo depth 1 seldepth 1 multipv 1 score cp 2 nodes 20 nps 20000 hashfull 0 tbhits 0 time 1 pv g1f3\ninfo depth 2 seldepth 2 multipv 1 score cp 2 nodes 40 nps 40000 hashfull 0 tbhits 0 time 1 pv g1f3\ninfo depth 3 seldepth 2 multipv 1 score cp 16 nodes 70 nps 70000 hashfull 0 tbhits 0 time 1 pv c2c3\ninfo depth 4 seldepth 2 multipv 1 score cp 29 nodes 101 nps 101000 hashfull 0 tbhits 0 time 1 pv e2e4\ninfo depth 5 seldepth 3 multipv 1 score cp 42 nodes 131 nps 131000 hashfull 0 tbhits 0 time 1 pv e2e4 g8f6\ninfo depth 6 seldepth 4 multipv 1 score cp 59 nodes 489 nps 244500 hashfull 0 tbhits 0 time 2 pv g1f3 d7d5 d2d4\ninfo depth 7 seldepth 6 multipv 1 score cp 31 nodes 1560 nps 390000 hashfull 1 tbhits 0 time 4 pv e2e4 d7d5 e4d5 d8d5 g1f3\ninfo depth 8 seldepth 6 multipv 1 score cp 40 nodes 2105 nps 421000 hashfull 1 tbhits 0 time 5 pv e2e4 d7d5 e4d5 d8d5\ninfo depth 9 seldepth 8 multipv 1 score cp 48 nodes 4500 nps 450000 hashfull 1 tbhits 0 time 10 pv e2e4 e7e5 g1f3 g8f6 f3e5 f6e4 d2d4 b8c6\ninfo depth 10 seldepth 10 multipv 1 score cp 200 nodes 7548 nps 471750 hashfull 2 tbhits 0 time 16 pv e2e4 e7e5 g1f3 g8f6 b1c3 d7d6 d2d4\nbestmove e2e4 ponder e7e5\n".to_string()
    }
}
