mod api;
mod config;
mod domain;
mod startup;
mod tasks;

use config::Settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::new();
    startup::run(settings).await
}
