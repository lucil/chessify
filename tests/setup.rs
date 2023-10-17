use chessify::startup::run;
use std::net::TcpListener;

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener
        .local_addr()
        .expect("Failed to retrieve local address")
        .port();
    let server = run(listener).expect("Failed to bind address");
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
