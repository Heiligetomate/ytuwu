use reqwest::RequestBuilder;

use crate::{models::response::Response, request::clients::body::RequestBody};

/// Wrapper type for request::clients::body::RequestBody
/// Created for better understanding of the code because RequestBuilder might be confusing
pub type ClientPrebuild = RequestBuilder;

/// This is a trait that defines all of the clients so they can be used for api requests
pub trait ClientWithHeaders {
    /// The response is important because the client has to define what response it expects
    type Response: Response;
    /// This function builds the ClientPrebuild which can then be called with the body
    /// Can be built like this where I is a generic representing the Id trait:
    /// I::Client::build_headers(client)
    fn build_headers(client: &reqwest::Client) -> ClientPrebuild;
    /// This function builds the body the youtube api expects
    /// Every client builds the requestbody that matches the correct parameters
    /// The visitor_data is an option because just some clients use it. Using it for clients that
    /// dont utilise it is not a problem.
    /// Can be built like this where I is a generic representing the Id trait:
    /// I::Client::build_body(None)
    fn build_body<'de>(_: &str, visitor_data: Option<String>) -> RequestBody<'de>;
}
