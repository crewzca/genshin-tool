use crate::model::enka_api::connect_api;
use crate::model::id_name::id_to_name;
use actix_web::{web, HttpResponse, Responder};
use log::info;
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
    context.insert("res", "3024");

    let renda = tera
        .render("top.html", &context)
        .expect("Teraのレンダリングに失敗しました");
    HttpResponse::Ok().content_type("text/html").body(renda)
}

pub async fn post_index(tera: web::Data<Tera>, form: web::Form<FormData>) -> impl Responder {
    info!("POST -start");
    let status = form.status.as_deref().unwrap_or("");
    let uuid = form.uuid.as_deref().unwrap_or("");
    let mut context = Context::new();
    context.insert("status", &status);
    context.insert("uuid", &uuid);

    if uuid.parse::<u64>().is_err() {
        context.insert("status", "UUIDの書式が不正です");
        info!("UUIDバリエーションエラー UUID:{}", uuid);
        let renda = tera
            .render("top.html", &context)
            .expect("Teraのレンダリングに失敗しました");
        HttpResponse::Ok().content_type("text/html").body(renda);
    }

    match connect_api(&uuid).await {
        Ok(mut response) => {
            info!("{:?}", &serde_json::json!(&response));
            context.insert("status", "");
            response = id_to_name(response).await;
            context.insert("res", &serde_json::json!(&response));
        }
        Err(e) => {
            context.insert("status", "UUIDがありませんでした");
            context.insert("res", "3024");
            info!("API接続エラー：{}", e);
        }
    }

    let renda = tera
        .render("top.html", &context)
        .expect("Teraのレンダリングに失敗しました");
    HttpResponse::Ok().content_type("text/html").body(renda)
}
