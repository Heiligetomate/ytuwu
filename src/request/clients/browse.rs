use crate::{
    models::FastBrowseResponse,
    request::clients::{
        client::{ClientPrebuild, ClientWithHeaders},
        endpoints::BROWSE_ENDPOINT,
        shared_params::{CLIENT_NAME_HEADER_NAME, CLIENT_VERSION_HEADER_NAME, CONTENT_TYPE_HEADER, GL, HL, ORIGIN_HEADER, TIME_ZONE, USER_AGENT_HEADER_NAME, UTC_OFFSET_MINUTES},
    },
};

const USER_AGENT: &str =
    "User-Agent: Mozilla/5.0 (Linux; Android 10; Quest 2) AppleWebKit/537.36 (KHTML, like Gecko) OculusBrowser/32.0.0.3.65 SamsungBrowser/4.3 Chrome/137.0.7151.61 Mobile VR Safari/537.36";
const X_CLIENT_NAME: &str = "28";
const X_CLIENT_VERSION: &str = "1.60.19";

const CLIENT_NAME: &str = "ANDROID_VR";
const CLIENT_VERSION: &str = "1.60.19";
const DEVICE_MAKE: &str = "Oculus";
const DEVICE_MODEL: &str = "Quest 2";

pub struct BrowseClient {}

impl ClientWithHeaders for BrowseClient {
    type Response = FastBrowseResponse;

    fn build_headers(client: &reqwest::Client) -> ClientPrebuild {
        client
            .post(BROWSE_ENDPOINT)
            .header(CONTENT_TYPE_HEADER.0, CONTENT_TYPE_HEADER.1)
            .header(USER_AGENT_HEADER_NAME, USER_AGENT)
            .header(CLIENT_NAME_HEADER_NAME, X_CLIENT_NAME)
            .header(CLIENT_VERSION_HEADER_NAME, X_CLIENT_VERSION)
            .header(ORIGIN_HEADER.0, ORIGIN_HEADER.1)
    }

    fn build_body<'de>(browse_id: &str, visitor_data: Option<String>) -> super::body::RequestBody<'de> {
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
                },
            },
            video_id: None,
            browse_id: Some(browse_id.to_owned()),
            url: None,
        }
    }
}
