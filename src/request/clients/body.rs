use serde::Serialize;

/// This is the body that every client uses
/// It should either hold a browse id OR a video id OR an url
/// video ids are mainly for the player client
/// browse ids are for every playlist and most channels
/// urls are for channel names such as @ntomusic
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody<'de> {
    pub context: Context<'de>,
    pub video_id: Option<String>,
    pub browse_id: Option<String>,
    pub url: Option<String>,
}

/// This struct is just a wrapper for the cliebt body
/// This is needed because youtubes expects this structure
#[derive(Serialize, Debug)]
pub struct Context<'de> {
    pub client: ClientBody<'de>,
}

/// This struct holds multiple values expected by youtube
/// Most of these values are always the same and therefore in a shared_params file
/// visitor_data is an option because it is just needed when using the playerclient
/// visitor_data is the data that youtube sends when there is a captcha that has to be solved
/// this captcha can be solved by sending the visitor_data back to youtube
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
    pub time_zone: &'de str,
    pub utc_offset_minutes: &'de str,
}
