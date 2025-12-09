use actix_web::web;

mod calculations;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(calculations::calculate);
}
