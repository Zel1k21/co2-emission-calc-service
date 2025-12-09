use actix_web::web;

pub mod calculations;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(calculations::calculate);
}
