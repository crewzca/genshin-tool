use crate::controller::top_controller;
use actix_web::{http::Method, web};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::method(Method::GET).to(top_controller::get_index))
            .route(web::method(Method::POST).to(top_controller::post_index)),
    );
}
