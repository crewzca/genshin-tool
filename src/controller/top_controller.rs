use actix_web::{web, HttpResponse, Responder};
use log::info;
use serde::Deserialize;
use tera::{Context, Tera};

#[derive(Deserialize)]
pub struct FormData {
    status: Option<String>,
    uuid: Option<u32>,
}

pub async fn get_index(tera: web::Data<Tera>) -> impl Responder {
    info!("GET -start");
    let mut context = Context::new();
    context.insert("status", "UUIDを入力してください");
    context.insert("uuid", &809308645);

    let renda = tera
        .render("top.html", &context)
        .expect("Teraのレンダリングに失敗しました");
    HttpResponse::Ok().content_type("text/html").body(renda)
}

pub async fn post_index(tera: web::Data<Tera>, form: web::Form<FormData>) -> impl Responder {
    info!("POST -start");
    let status = form.status.as_deref().unwrap_or("UUIDの書式が不正です");
    let uuid = form.uuid.unwrap_or(0);

    let mut context = Context::new();
    context.insert("status", &status);
    context.insert("uuid", &uuid);

    let renda = tera
        .render("top.html", &context)
        .expect("Teraのレンダリングに失敗しました");
    HttpResponse::Ok().content_type("text/html").body(renda)
}
