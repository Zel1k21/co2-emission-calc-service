use crate::domain::{CalcRequest, CalcResponse, calculate_emission};
use actix_web::{HttpResponse, post, web};

#[post("/calculateEmission")]
async fn calculate(req: web::Json<CalcRequest>) -> HttpResponse {
    let value = calculate_emission(&req).await;
    HttpResponse::Ok().json(CalcResponse {
        calculation_result: value,
    })
}
