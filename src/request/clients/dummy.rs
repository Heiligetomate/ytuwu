use crate::{
    models::DummyResponse,
    request::clients::client::{ClientPrebuild, ClientWithHeaders},
};

pub struct DummyClient {}

impl ClientWithHeaders for DummyClient {
    type Response = DummyResponse;

    fn build_headers(_: &reqwest::Client) -> ClientPrebuild {
        panic!("this is a dummy and should never be called");
    }

    fn build_body<'de>(_: &str, _: Option<String>) -> super::body::RequestBody<'de> {
        panic!("this is a dummy and should never be called");
    }
}
