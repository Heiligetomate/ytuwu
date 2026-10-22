use crate::{
    models::PlayerResponse,
    request::clients::{
        client::{ClientPrebuild, ClientWithHeaders},
        endpoints::PLAYER_ENDPOINT,
        shared_params::{CLIENT_NAME_HEADER_NAME, CLIENT_VERSION_HEADER_NAME, CONTENT_TYPE_HEADER, GL, HL, ORIGIN_HEADER, TIME_ZONE, USER_AGENT_HEADER_NAME, UTC_OFFSET_MINUTES},
    },
};
const USER_AGENT: &str = "com.google.android.apps.youtube.vr.oculus/1.65.10 (Linux; U; Android 12L; eureka-user Build/SQ3A.220605.009.A1) gzip";
const X_CLIENT_NAME: &str = "3";
const X_CLIENT_VERSION: &str = "1.65.10";

const CLIENT_NAME: &str = "ANDROID_VR";
const CLIENT_VERSION: &str = "1.65.10";
const DEVICE_MAKE: &str = "Oculus";
const DEVICE_MODEL: &str = "Quest 3";

/// This client is for getting the streams to download media
/// It should be used with the Video Id
/// It returns a PlayerResponse
pub struct PlayerClient {}

impl ClientWithHeaders for PlayerClient {
    type Response = PlayerResponse;

    fn build_headers(client: &reqwest::Client) -> ClientPrebuild {
        client
            .post(PLAYER_ENDPOINT)
            .header(CONTENT_TYPE_HEADER.0, CONTENT_TYPE_HEADER.1)
            .header(USER_AGENT_HEADER_NAME, USER_AGENT)
            .header(CLIENT_NAME_HEADER_NAME, X_CLIENT_NAME)
            .header(CLIENT_VERSION_HEADER_NAME, X_CLIENT_VERSION)
            .header(ORIGIN_HEADER.0, ORIGIN_HEADER.1)
    }

    fn build_body<'de>(video_id: &str, visitor_data: Option<String>) -> super::body::RequestBody<'de> {
        super::body::RequestBody {
            context: super::body::Context {
                client: super::body::ClientBody {
                    client_name: CLIENT_NAME,
                    client_version: CLIENT_VERSION,
                    device_make: Some(DEVICE_MAKE),
                    device_model: Some(DEVICE_MODEL),
                    hl: HL,
                    gl: GL,
                    time_zone: TIME_ZONE,
                    utc_offset_minutes: UTC_OFFSET_MINUTES,
                    visitor_data,
                    android_sdk_version: Some(32),
                },
            },
            video_id: Some(video_id.to_owned()),
            browse_id: None,
            url: None,
        }
    }
}
