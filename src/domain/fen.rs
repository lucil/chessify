use serde::{Deserialize, Serialize};
use urlencoding;

#[derive(Serialize, Deserialize, Debug)]
pub struct Fen {
    pub code: String,
}

#[derive(Debug)]
pub enum FenError {
    InvalidFen(String),
    DecodingError(String),
}

impl Fen {
    pub fn new(code: &str) -> Result<Fen, FenError> {
        if code.trim().is_empty() {
            return Err(FenError::InvalidFen("FEN string is empty".to_string()));
        }

        Ok(Fen {
            code: code.to_string(),
        })
    }

    pub fn encode(&self) -> String {
        urlencoding::encode(&self.code).into_owned()
    }

    pub fn decode(encoded: &str) -> Result<String, FenError> {
        urlencoding::decode(encoded)
            .map(|decoded| decoded.into_owned())
            .map_err(|_| FenError::DecodingError("Failed to decode FEN string".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::assert_err;

    #[test]
    fn new_fen_is_created() {
        let fen = Fen::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        assert_eq!(
            fen.unwrap().code,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        );
    }

    #[test]
    fn throws_error_when_empty_code() {
        assert_err!(Fen::new(""));
    }

    #[test]
    fn fen_gets_encoded() {
        let fen = Fen::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
        let encoded = fen.encode();
        assert_eq!(
            encoded,
            "rnbqkbnr%2Fpppppppp%2F8%2F8%2F8%2F8%2FPPPPPPPP%2FRNBQKBNR"
        );
    }

    #[test]
    fn fen_gets_decoded() {
        let fen = Fen::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
        let encoded = fen.encode();
        let decoded = Fen::decode(&encoded).unwrap();
        assert_eq!(decoded, fen.code);
    }
}
