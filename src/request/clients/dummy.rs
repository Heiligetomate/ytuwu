use crate::{
    models::dummy::DummyResponse,
    request::clients::client::{ClientPrebuild, ClientWithHeaders},
};

pub struct DummyClient {}

impl ClientWithHeaders for DummyClient {
    type Response = DummyResponse;

    fn build_headers(client: &reqwest::Client) -> ClientPrebuild {
        panic!("this is a dummy and should never be called");
    }

    fn build_body<'de>(_: &str, _: Option<String>) -> super::body::RequestBody<'de> {
        super::body::RequestBody {
            context: super::body::Context {
                client: super::body::ClientBody {
                    client_name: "",
                    client_version: "",
                    device_make: None,
                    device_model: None,
                    hl: "",
                    gl: "",
                    time_zone: "",
                    utc_offset_minutes: "",
                    visitor_data: None,
                },
            },
            video_id: None,
            browse_id: None,
            url: None,
        }
    }
}
