use rand::Rng;
use tokio::time::{Duration, sleep};
use validator::Validate;

pub async fn calculate_emission(req: &crate::domain::models::CalcRequest) -> f32 {
    req.validate().unwrap();
    if req.auth_token.is_empty() {
        return 0.0;
    }
    // Set random delay from 5 to 10 seconds
    let delay_secs = rand::thread_rng().gen_range(5..=10);
    let mut result = 0.0;
    for (input, constant) in req.input_fields.iter().zip(req.stage_constants.iter()) {
        result += input * constant;
    }
    sleep(Duration::from_secs(delay_secs)).await;
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::CalcRequest;

    #[tokio::test]
    async fn test_calculation() {
        let request = CalcRequest {
            auth_token: "test_token".to_string(),
            input_fields: vec![1.0, 2.0, 3.0],
            stage_constants: vec![0.5, 1.0, 1.5],
        };
        let result = calculate_emission(&request).await;
        assert_eq!(result, 7.0);
    }
}
