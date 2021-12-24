mod config;
mod models;

use crate::models::Status;
use actix_web::{HttpServer, App, web, Responder};
use std::io;
use dotenv::dotenv;

async fn status() -> impl Responder {
  web::HttpResponse::Ok()
    .json(Status { status: "Rodando carai!".to_string() })
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
  dotenv().ok();

  let config = crate::config::Config::from_env().unwrap();
  
  HttpServer::new(|| {
    App::new()
      .route("/", web::get().to(status))
  })
  .bind(format!("{}:{}", config.server.host, config.server.port))?
  .run()
  .await
}
