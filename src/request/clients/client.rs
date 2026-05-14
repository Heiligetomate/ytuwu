use reqwest::RequestBuilder;

use crate::{request::clients::body::RequestBody, shared_traits::Response};

pub type ClientPrebuild = RequestBuilder;

pub trait ClientWithHeaders {
    type Response: Response;
    fn build_headers() -> ClientPrebuild;
    fn build_body<'de>(_: &str, visitor_data: Option<String>) -> RequestBody<'de>;
}
