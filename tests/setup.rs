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
    println!("{:?}", port);
    format!("http://127.0.0.1:{}", port)
}

pub async fn execute_get(url: &str) -> reqwest::Response {
    let address = spawn_app();
    let client = reqwest::Client::new();
    client
        .get(&format!("{}{}", &address, url))
        .send()
        .await
        .expect("Failed to execute request.")
}
