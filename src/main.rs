use chessify::startup::run;
use std::net::TcpListener;

#[cfg(not(tarpaulin_include))]
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let address = TcpListener::bind("127.0.0.1:8000")?;
    run(address)?.await
}
