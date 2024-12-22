use crate::model::enka_api::connect_api;
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use serde::Deserialize;
use tera::{Context, Tera};

#[derive(Deserialize)]
pub struct FormData {
    status: Option<String>,
    uuid: Option<String>,
}

pub async fn get_index(tera: web::Data<Tera>) -> impl Responder {
    info!("GET -start");
    let mut context = Context::new();
    context.insert("status", "UUIDを入力してください");
    context.insert("uuid", "809308645");
    context.insert("nickname", "");
    context.insert("level", "");

    let renda = tera
        .render("top.html", &context)
        .expect("Teraのレンダリングに失敗しました");
    HttpResponse::Ok().content_type("text/html").body(renda)
}

pub async fn post_index(tera: web::Data<Tera>, form: web::Form<FormData>) -> impl Responder {
    info!("POST -start");
    let status = form.status.as_deref().unwrap_or("UUIDの書式が不正です");
    let uuid = form.uuid.as_deref().unwrap_or("");
    let mut context = Context::new();

    match connect_api(&uuid).await {
        Ok(response) => {
            context.insert("nickname", &response.player_info.nickname);
            context.insert("level", &response.player_info.level.to_string());
            if !uuid.eq(&response.uid) {
                error!("改ざんを検知しました");
            }
        }
        Err(e) => {
            context.insert("nickname", "");
            error!("{}", e)
        }
    }

    context.insert("status", &status);
    context.insert("uuid", &uuid);

    let renda = tera
        .render("top.html", &context)
        .expect("Teraのレンダリングに失敗しました");
    HttpResponse::Ok().content_type("text/html").body(renda)
}
