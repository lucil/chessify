use mockall::automock;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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
        let output_vec: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

        // Start a separate thread to read from the child process's output continuously
        let child_stdout = child.stdout.take().expect("Child process has no stdout");

        let stdout_thread = {
            let output_vec = Arc::clone(&output_vec);
            thread::spawn(move || {
                let mut output = String::new();
                let mut child_stdout = child_stdout;

                loop {
                    match child_stdout.read_to_string(&mut output) {
                        Ok(0) => break, // End of output
                        Ok(_) => {
                            // Process 'output' as needed
                            output_vec.lock().unwrap().push(output.clone());
                            output.clear();
                        }
                        Err(_) => break, // Error reading
                    }
                }
            })
        };

        // Start a separate thread to write data to the child process's input continuously
        let mut child_stdin = child.stdin.take().expect("Child process has no stdin");
        let stdin_thread = {
            let _output_vec = Arc::clone(&output_vec);
            thread::spawn(move || {
                for command in commands {
                    child_stdin
                        .write_all(command.as_bytes())
                        .expect("Failed to write data to child");
                    thread::sleep(Duration::from_millis(500)); // Sleep between writes
                }
            })
        };

        // Wait for the stdout and stdin threads to finish
        stdout_thread.join().expect("Failed to join stdout thread");
        stdin_thread.join().expect("Failed to join stdin thread");

        //let _ = child.kill();

        Arc::try_unwrap(output_vec)
            .expect("Failed to unwrap Arc")
            .into_inner()
            .expect("Failed to get inner value")
            .join("-")
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
            commands::go_depth(10),
        ];
        let result = stockfish.execute(commands_to_execute);

        assert!(result.contains("score cp"));
    }
}
