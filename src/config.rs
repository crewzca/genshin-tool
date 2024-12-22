use crate::controller;
use actix_web::{http::Method, web};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::method(Method::GET).to(controller::get_index))
            .route(web::method(Method::POST).to(controller::post_index)),
    );
}
