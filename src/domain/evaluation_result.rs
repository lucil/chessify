use super::Fen;
use mockall::automock;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EvaluationResult {
    pub fen: Fen,
    pub score: f32,
}

impl EvaluationResult {
    pub fn new(fen_code: &str, score: f32) -> Result<Self, String> {
        let fen = Fen::new(fen_code).map_err(|_| "Failed to create EvaluationResult: {:?}")?;
        return Ok(EvaluationResult { fen, score });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_evaluation_result_is_created() {
        let fen_code = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let evaluation_result = EvaluationResult::new(fen_code, 0.0).unwrap();
        assert_eq!(
            evaluation_result.fen.code,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        );
        assert_eq!(evaluation_result.score, 0.0);
    }

    #[test]
    fn new_evaluation_result_handles_fen_error() {
        let fen_code = ""; // Empty FEN
        let result = EvaluationResult::new(fen_code, 0.0);
        assert!(result.is_err());
    }
}
