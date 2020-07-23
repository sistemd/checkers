use actix::prelude::*;
use actix_files;
use actix_web::web;
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer, Responder};
use std::fs;
use std::io::prelude::*;

mod checkers;
mod game_master;
mod ws_server;

pub async fn index() -> impl Responder {
    let mut file = fs::File::open("www/index.html").unwrap();
    let mut body = String::new();
    file.read_to_string(&mut body).unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let gm = game_master::GameMaster::new().start();
    let player_id_counter = web::Data::new(ws_server::PlayerIDCounter::new());
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .service(actix_files::Files::new("dist/", "www/dist"))
            .configure(ws_server::config_with(
                gm.clone(),
                player_id_counter.clone(),
            ))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {}
