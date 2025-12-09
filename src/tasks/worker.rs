use crate::domain::send_result_to_backend;
use crate::domain::{CalcRequest, calculate_emission};
use tokio::spawn;

pub fn spawn_calculation_task(req: CalcRequest) {
    spawn(async move {
        let result = calculate_emission(&req).await;
        if let Err(err) = send_result_to_backend(req.id, result).await {
            eprintln!("Failed to send result to backend: {:?}", err);
        }
    });
}
