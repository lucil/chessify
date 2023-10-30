use crate::domain::Fen;

pub fn position_fen(fen: &Fen) -> String {
    format!("position fen {}\n", fen.code)
}

pub fn go_depth(depth: i32) -> String {
    format!("go depth {}\n", depth)
}

pub fn uci() -> String {
    "uci\n".to_string()
}

pub fn isready() -> String {
    "isready\n".to_string()
}

#[cfg(test)]
mod commands_tests {
    use super::*;

    #[test]
    fn returns_uci_command() {
        let expected = "uci\n";
        let actual = uci();
        assert_eq!(actual, expected);
    }

    #[test]
    fn returns_isready_command() {
        let expected = "isready\n";
        let actual = isready();
        assert_eq!(actual, expected);
    }

    #[test]
    fn returns_position_fen_command() {
        let fen = Fen::new("some_fen").unwrap();
        let expected = format!("position fen {}\n", fen.code);
        let actual = position_fen(&fen);
        assert_eq!(actual, expected);
    }

    #[test]
    fn returns_go_depth_command() {
        let depth = 10;
        let expected = "go depth 10\n";
        let actual = go_depth(depth);
        assert_eq!(actual, expected);
    }
}
