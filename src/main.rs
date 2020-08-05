use actix::prelude::*;
use actix_files;
use actix_web::web;
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer, Responder};
use pnet::datalink;
use pnet::ipnetwork::IpNetwork;
use std::fs;
use std::io::prelude::*;
use std::net::Ipv4Addr;

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
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .service(actix_files::Files::new("dist/", "www/dist"))
            .configure(ws_server::config_with(
                gm.clone(),
                player_id_counter.clone(),
            ))
    });

    let port = 8080;

    for ip in ips() {
        server = server.bind((ip, port)).unwrap();
    }

    println!("Listening on {}.", ips_string(port));

    server.run().await
}

fn ips() -> impl Iterator<Item = Ipv4Addr> {
    datalink::interfaces()
        .into_iter()
        .map(|iface| {
            iface.ips.into_iter().filter_map(|ip| match ip {
                IpNetwork::V4(v4) => Some(v4.ip()),
                IpNetwork::V6(_) => None,
            })
        })
        .flatten()
}

fn ips_string(port: u16) -> String {
    ips()
        .map(|ip| format!("http://{}:{}", ip, port))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {}
