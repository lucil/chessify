#[derive(Debug)]
pub struct Fen {
    //implement a field
    pub code: String,
}

impl Fen {
    //implement new function
    pub fn new(code: &str) -> Result<Fen, std::io::Error> {
        if code.trim().is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "FEN string is empty",
            ));
        }

        return Ok(Fen {
            code: code.to_string(),
        });
    }

    //implement encode function
    pub fn encode(&self) -> String {
        base64::encode(&self.code)
    }

    //implement decode function
    pub fn decode(encoded: String) -> String {
        let decoded = base64::decode(encoded).unwrap();
        String::from_utf8(decoded).unwrap()
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
    fn fen_gets_encoded_to_base64() {
        let fen = Fen::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
        let encoded = fen.encode();
        assert_eq!(
            encoded,
            "cm5icWtibnIvcHBwcHBwcHAvOC84LzgvOC9QUFBQUFBQUC9STkJRS0JOUg=="
        );
    }

    #[test]
    fn fen_gets_decoded_from_base_64() {
        let fen = Fen::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
        let encoded = fen.encode();
        let decoded = Fen::decode(encoded);
        assert_eq!(decoded, fen.code);
    }
}
