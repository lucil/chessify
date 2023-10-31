use crate::domain::Score;

pub fn parse_go_depth_result(execution_result: &str) -> Score {
    let mut last_cp_score: Option<Score> = None;

    for line in execution_result.lines() {
        if let Some(score) = line.strip_prefix("info depth") {
            let score_parts: Vec<&str> = score.split("score cp ").collect();
            if score_parts.len() > 1 {
                if let Ok(cp) = score_parts[1]
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .parse::<String>()
                {
                    last_cp_score = Some(Score::parse_string(&cp).unwrap());
                }
            }
        }
    }

    last_cp_score.unwrap().normalise()
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use crate::domain::Score;
    #[test]
    fn parses_stockfish_result() {
        let stockfish_result = "info depth 10 seldepth 7 multipv 1 score cp -729 nodes 8982 nps 374250 hashfull 6 tbhits 0 time 24 pv h2h3 c7c2 d1c1 c2a2 c1c7 b8b3\nbestmove h2h3 ponder c7c2";

        let parsed_result = parse_go_depth_result(stockfish_result);
        assert_eq!(parsed_result, Score::new(-7.29))
    }
}
