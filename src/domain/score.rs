use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Score {
    pub value: f64,
}

impl Score {
    pub fn new(input: f64) -> Self {
        Score { value: input }
    }
    pub fn parse_string(input: &str) -> Result<Score, String> {
        let result = f64::from_str(input);
        match result {
            Ok(value) => Ok(Score::new(value)),
            Err(error) => Err(format!("Error parsing score: {}", error)),
        }
    }

    pub fn normalise(&self) -> Self {
        Score {
            value: self.value / 100.0,
        }
    }
}

#[cfg(test)]
mod score_tests {

    use super::*;
    #[test]
    fn new_score_is_created() {
        let score = Score::new(1.0);
        assert_eq!(score.value, 1.0);
    }

    #[test]
    fn string_is_parsed() {
        let string_value = "1.0";
        let score = Score::parse_string(string_value);

        assert_eq!(score.unwrap().value, 1.0);
    }

    #[test]
    fn string_is_not_parsed() {
        let string_value = "not_a_valid_score";
        let score = Score::parse_string(string_value);

        assert!(score.err().unwrap().contains("Error parsing score"));
    }

    #[test]
    fn normalises_score() {
        let string_value = "50";
        let score = Score::parse_string(string_value);
        let normalised_score = score.unwrap().normalise();
        assert_eq!(normalised_score.value, 0.5);
    }
}
