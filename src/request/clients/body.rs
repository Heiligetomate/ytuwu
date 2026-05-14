use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody<'de> {
    pub context: Context<'de>,
    pub video_id: Option<String>,
    pub browse_id: Option<String>,
    // content_check_ok: bool,
    // racy_check_ok: bool,
}

#[derive(Serialize, Debug)]
pub struct Context<'de> {
    pub client: ClientBody<'de>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientBody<'de> {
    pub client_name: &'de str,
    pub client_version: &'de str,
    pub device_make: Option<&'de str>,
    pub device_model: Option<&'de str>,
    pub hl: &'de str,
    pub gl: &'de str,
    pub visitor_data: Option<String>,
}
