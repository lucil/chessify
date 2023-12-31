use crate::routes::{fen_score, health_check};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .service(web::resource("fen/score/{fen_string}").route(web::get().to(fen_score)))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
