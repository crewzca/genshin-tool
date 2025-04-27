mod config;
mod controller;
mod entity;
mod model;

use crate::config::config_app;
use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenvy::dotenv;
use log::{debug, error};
use std::env::var;
use std::process::exit;
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let env = var("APP_ENV").unwrap();

    let tera = match Tera::new("templates/**/*.html") {
        Ok(mut t) => {
            t.autoescape_on(vec![".html"]);
            if env.eq("DEV") {
                debug!("フルリロードを開始します");
                t.full_reload().expect("フルリロードに失敗しました");
            }
            t
        }
        Err(e) => {
            error!("Teraのレンダリングに失敗しました {}", e);
            exit(1);
        }
    };

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
