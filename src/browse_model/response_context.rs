use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseContext {
    pub visitor_data: Option<String>,
}
