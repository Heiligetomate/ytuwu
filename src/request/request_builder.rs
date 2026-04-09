use serde::{Deserialize, Serialize};

use crate::id_resolver::{BrowseId, Id, VideoId};

 
#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    context: Context,
    video_id: Option<String>,
    browse_id: Option<String>,
    content_check_ok: bool,
    racy_check_ok: bool,
}

impl RequestBody {
    fn new(video_id: Option<String>, browse_id: Option<String>, visitor_data: Option<String>) -> Self { 
        Self {
            context          : Context::default_downloader_body(visitor_data),
            video_id         : video_id,
            browse_id        : browse_id,
            content_check_ok : true,
            racy_check_ok    : true,
        }
    }
    pub fn new_browse_request(browse_id: BrowseId, visitor_data: Option<String>) -> Self {
        RequestBody::new(None, Some(browse_id.get_id()), visitor_data)
    }
    pub fn new_player_request(video_id: VideoId, visitor_data: Option<String>) -> Self {
        RequestBody::new(Some(video_id.get_id()), None, visitor_data)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Context {
    client: Client,
}


#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    client_name: String,
    client_version: String,
    device_make: String,
    device_model: String,
    android_sdk_version: u16,
    hl: String,
    gl: String,
    time_zone: String,
    utc_offset_minutes: u16,
    visitor_data: Option<String>,
}



impl Context {
    pub fn default_downloader_body(visitor_data: Option<String>) -> Self {
        Self {
            client: Client {             
                client_name: String::from("ANDROID_VR"),
                client_version: String::from("1.60.19"),
                device_make: String::from("Oculus"),
                device_model: String::from("Quest 2"),
                android_sdk_version: 29,
                hl: String::from("en"),
                gl: String::from("US"),
                time_zone: String::from("UTC"),
                utc_offset_minutes: 0,
                visitor_data: visitor_data,
            }
        }
    }
}

