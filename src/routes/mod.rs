use actix_web::web;
use crate::controllers;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
        .configure(controllers::user_controller::init)
        .configure(controllers::ledger_controller::init));
}