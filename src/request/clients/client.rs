use reqwest::RequestBuilder;

use crate::{models::response::Response, request::clients::body::RequestBody};

pub type ClientPrebuild = RequestBuilder;

pub trait ClientWithHeaders {
    type Response: Response;
    fn build_headers(client: &reqwest::Client) -> ClientPrebuild;
    fn build_body<'de>(_: &str, visitor_data: Option<String>) -> RequestBody<'de>;
}
