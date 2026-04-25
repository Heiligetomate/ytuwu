use crate::id_resolver::{BrowseId, Id, VideoId};
use crate::request::parameters::*;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody<'de> {
    context: Context<'de>,
    video_id: Option<String>,
    browse_id: Option<String>,
    content_check_ok: bool,
    racy_check_ok: bool,
}

impl<'de> RequestBody<'de> {
    fn new(
        video_id: Option<String>,
        browse_id: Option<String>,
        visitor_data: Option<String>,
    ) -> Self {
        Self {
            context: Context::default_downloader_body(visitor_data),
            video_id: video_id,
            browse_id: browse_id,
            content_check_ok: true,
            racy_check_ok: true,
        }
    }
    pub fn new_browse_request(browse_id: BrowseId, visitor_data: Option<String>) -> Self {
        RequestBody::new(None, Some(browse_id.get_id()), visitor_data)
    }
    pub fn new_player_request(video_id: VideoId, visitor_data: Option<String>) -> Self {
        RequestBody::new(Some(video_id.get_id()), None, visitor_data)
    }
}

#[derive(Serialize, Debug)]
pub struct Context<'de> {
    client: Client<'de>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Client<'de> {
    client_name: &'de str,
    client_version: &'de str,
    device_make: &'de str,
    device_model: &'de str,
    android_sdk_version: u16,
    hl: &'de str,
    gl: &'de str,
    time_zone: &'de str,
    utc_offset_minutes: u16,
    visitor_data: Option<String>,
}

impl<'de> Context<'de> {
    pub fn default_downloader_body(visitor_data: Option<String>) -> Self {
        Self {
            client: Client {
                client_name: CLIENT_NAME,
                client_version: CLIENT_VERSION,
                device_make: DEVICE_MAKE,
                device_model: DEVICE_MODEL,
                android_sdk_version: ANDROID_SDK_VERSION,
                hl: HL,
                gl: GL,
                time_zone: TIMEZONE,
                utc_offset_minutes: UTC_OFFSET_MINUTES,
                visitor_data: visitor_data,
            },
        }
    }
}
