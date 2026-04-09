use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseContext {
    pub visitor_data: Option<String>,
    max_age_seconds: Option<u64>,
    rollout_token: Option<String>,
    service_tracking_params: Vec<ServiceTrackingParams>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceTrackingParams {
    service: String,
    params: Vec<Param>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Param {
    key: String,
    value: String,
}


