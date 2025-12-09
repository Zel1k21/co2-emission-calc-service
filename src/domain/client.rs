use crate::domain::models::CalcResponse;
use reqwest::Client;

pub async fn send_result_to_backend(id: i32, value: f32) -> Result<(), reqwest::Error> {
    let client = Client::new();

    client
        .put("http://localhost:8082/api/stage-requests/asyncUpdateCalculation")
        .json(&CalcResponse {
            request_id: id,
            calculation_result: value,
        })
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
