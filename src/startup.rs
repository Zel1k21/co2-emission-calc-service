use crate::{api, config::Settings};
use actix_web::{App, HttpServer};

pub async fn run(settings: Settings) -> std::io::Result<()> {
    HttpServer::new(move || App::new().configure(api::init))
        .bind((settings.host.clone(), settings.port))?
        .run()
        .await
}
