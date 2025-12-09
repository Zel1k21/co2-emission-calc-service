use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct CalcRequest {
    pub id: i32,

    #[validate(length(min = 1, message = "Auth token must not be empty"))]
    pub auth_token: String,

    #[validate(length(min = 1, message = "Input fields must not be empty"))]
    pub input_fields: Vec<f32>,

    #[validate(length(min = 1, message = "Stage constants must not be empty"))]
    pub stage_constants: Vec<f32>,
}

#[derive(Serialize)]
pub struct ApiMessage {
    pub message: String,
}

#[derive(Serialize)]
pub struct CalcResponse {
    pub request_id: i32,
    pub calculation_result: f32,
}
