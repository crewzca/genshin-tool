use crate::model::enka_api::connect_api;
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

#[derive(Deserialize)]
pub struct FormData {
    status: Option<String>,
    uuid: Option<String>,
}

#[derive(Serialize, Default)]
pub struct User {
    nickname: String,
    level: u32,
}

#[derive(Serialize, Default)]
pub struct Chara {
    atk: u32,
}

pub async fn get_index(tera: web::Data<Tera>) -> impl Responder {
    info!("GET -start");
    let mut context = Context::new();
    let user = User::default();
    context.insert("status", "UUIDを入力してください");
    context.insert("uuid", "809308645");
    context.insert("user", &user);

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
    let mut user = User::default();
    context.insert("status", &status);
    context.insert("uuid", &uuid);

    if uuid.parse::<u64>().is_err() {
        context.insert("status", "UUIDの書式が不正です");
        context.insert("user", &user);
        info!("UUIDバリエーションエラー UUID:{}", uuid);
        let renda = tera
            .render("top.html", &context)
            .expect("Teraのレンダリングに失敗しました");
        HttpResponse::Ok().content_type("text/html").body(renda);
    }

    match connect_api(&uuid).await {
        Ok(response) => {
            user.nickname = response.player_info.nickname;
            user.level = response.player_info.level;
            context.insert("status", "");
            if !uuid.eq(&response.uid) {
                error!("改ざんを検知しました");
            }
        }
        Err(e) => {
            context.insert("status", "UUIDがありませんでした");

            error!("API接続エラー：{}", e);
        }
    }
    context.insert("user", &user);
    let renda = tera
        .render("top.html", &context)
        .expect("Teraのレンダリングに失敗しました");
    HttpResponse::Ok().content_type("text/html").body(renda)
}
