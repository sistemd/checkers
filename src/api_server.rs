use actix_web::web;
use actix_web::Responder;
use serde::Serialize;

const API_ROOT: &str = "/api";

#[derive(Serialize)]
struct Sample {
    a: String,
}

async fn example() -> impl Responder {
    web::Json(Sample { a: "hi".to_owned() })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope(API_ROOT).route("/example", web::get().to(example)));
}
