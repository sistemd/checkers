use actix::prelude::*;
use actix_web::{middleware::Logger, App, HttpServer};

mod api_server;
mod checkers;
mod game_master;
mod www_server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    game_master::GameMaster::new().start();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(api_server::config)
            .configure(www_server::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
