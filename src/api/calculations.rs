use crate::domain::{ApiMessage, CalcRequest};
use crate::tasks::spawn_calculation_task;
use actix_web::{HttpResponse, post, web};

#[post("/calculateEmission")]
async fn calculate(req: web::Json<CalcRequest>) -> HttpResponse {
    spawn_calculation_task(req.into_inner());
    HttpResponse::Accepted().json(ApiMessage {
        message: "Data received successfully".into(),
    })
}
