mod config;
mod controller;
mod entity;
mod model;

use crate::config::config_app;
use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let tera = Tera::new("templates/**/*.html").expect("Tera読み込み失敗");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(Files::new("/static", "static").show_files_listing())
            .wrap(Logger::default())
            .configure(config_app)
    })
    .bind("127.0.0.1:8080")
    .expect("アドレスのバインドに失敗")
    .run()
    .await
}
