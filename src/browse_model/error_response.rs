use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error_code: u16,
    pub message: String, 
}
