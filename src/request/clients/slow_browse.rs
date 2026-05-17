use crate::{
    models::slow_browse::SlowAlbumResponse,
    request::clients::{
        body::RequestBody,
        client::{ClientPrebuild, ClientWithHeaders},
        endpoints::BROWSE_ENDPOINT,
        shared_params::{CLIENT_NAME_HEADER_NAME, CLIENT_VERSION_HEADER_NAME, CONTENT_TYPE_HEADER, GL, HL, ORIGIN_HEADER, TIME_ZONE, USER_AGENT_HEADER_NAME, UTC_OFFSET_MINUTES},
    },
};

const USER_AGENT: &str = "User-Agent: Mozilla/5.0 (Linux; Android 10; Quest 2) AppleWebKit/537.36 (KHTML, like Gecko) OculusBrowser/32.0.0.3.65 SamsungBrowser/4.3 Chrome/137.0.7151.61 Mobile VR Safari/537.36";
const X_CLIENT_NAME: &str = "67";
const X_CLIENT_VERSION: &str = "1.20260428.11.00";

const CLIENT_NAME: &str = "WEB_REMIX";
const CLIENT_VERSION: &str = "1.20260428.11.00";

pub struct SlowBrowseClient {}

impl ClientWithHeaders for SlowBrowseClient {
    type Response = SlowAlbumResponse;

    fn build_headers() -> ClientPrebuild {
        let client = reqwest::Client::new();

        client
            .post(BROWSE_ENDPOINT)
            .header(USER_AGENT_HEADER_NAME, USER_AGENT)
            .header(CONTENT_TYPE_HEADER.0, CONTENT_TYPE_HEADER.1)
            .header(CLIENT_NAME_HEADER_NAME, X_CLIENT_NAME)
            .header(CLIENT_VERSION_HEADER_NAME, X_CLIENT_VERSION)
            .header(ORIGIN_HEADER.0, ORIGIN_HEADER.1)
    }

    fn build_body<'de>(browse_id: &str, visitor_data: Option<String>) -> RequestBody<'de> {
        RequestBody {
            context: super::body::Context {
                client: super::body::ClientBody {
                    client_name: CLIENT_NAME,
                    client_version: CLIENT_VERSION,
                    device_make: None,
                    device_model: None,
                    hl: HL,
                    gl: GL,
                    visitor_data,
                    time_zone: TIME_ZONE,
                    utc_offset_minutes: UTC_OFFSET_MINUTES,
                },
            },
            video_id: None,
            browse_id: Some(browse_id.to_owned()),
        }
    }
}
