use mockall::automock;

#[automock]
pub trait Engine {
    fn evaluate_fen(&self, fen: &str) -> f32;
}

pub struct StockfishEngine {}

impl Engine for StockfishEngine {
    fn evaluate_fen(&self, _fen: &str) -> f32 {
        1.0
    }
}

impl StockfishEngine {
    pub fn new() -> StockfishEngine {
        StockfishEngine {}
    }
}

impl Default for StockfishEngine {
    fn default() -> Self {
        StockfishEngine::new()
    }
}
