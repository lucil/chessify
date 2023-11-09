use mockall::automock;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use tokio::time::Duration;

pub trait Engine {
    fn execute(&self, commands: Vec<String>) -> String;
}

pub struct Stockfish {}

#[automock]
impl Engine for Stockfish {
    fn execute(&self, commands: Vec<String>) -> String {
        let mut child = Command::new("stockfish")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start Stockfish");

        let mut child_stdin = child.stdin.take().expect("Child process has no stdin");
        let child_stdout = child.stdout.take().expect("Child process has no stdout");

        let buf_reader = BufReader::new(child_stdout);
        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

        let mut line = String::new();

        let thread_tx = tx.clone();

        let _ = thread::spawn(move || {
            let mut buf_reader = buf_reader;

            loop {
                let _ = buf_reader.read_line(&mut line);

                thread_tx.send(line.clone()).expect("Unable to send line");
            }
        });

        let timeout = Duration::new(2, 0);
        let mut output = String::new();
        for command in commands {
            println!("Sending {:?}", command);
            child_stdin
                .write_all(command.as_bytes())
                .expect("Failed to write data to child");

            loop {
                let result = rx.recv_timeout(timeout);
                if result.is_err() {
                    break;
                }

                output = result.unwrap();
            }
        }

        let _ = child.kill();

        output
    }
}

impl Stockfish {
    pub fn new() -> Stockfish {
        Stockfish {}
    }
}

impl Default for Stockfish {
    fn default() -> Self {
        Stockfish::new()
    }
}

#[cfg(test)]
mod stockfish_tests {
    use crate::domain::Fen;
    use crate::engine::commands;

    use super::*;

    #[test]
    fn executes_multiple_commands() {
        let stockfish = Stockfish::new();
        let commands_to_execute: Vec<String> = vec![commands::uci(), commands::isready()];
        let result = stockfish.execute(commands_to_execute);
        assert!(result.contains("uciok"));
        assert!(result.contains("readyok"));
    }

    #[test]
    fn can_evaluate_fen_position() {
        let stockfish = Stockfish::new();
        let commands_to_execute: Vec<String> = vec![
            commands::position_fen(
                &Fen::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap(),
            ),
            commands::go_depth(25),
        ];
        let result = stockfish.execute(commands_to_execute);
        //println!("{:?}", result);
        assert!(result.contains("score cp"));
        assert!(result.contains("info depth 25"));
    }
}
