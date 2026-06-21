use crate::{
    models::DummyResponse,
    request::clients::client::{ClientPrebuild, ClientWithHeaders},
};

/// This client exists because of traits expecting a Client as a type  
/// This will panic if this client is actually used
/// Just use this an actual dummy
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
