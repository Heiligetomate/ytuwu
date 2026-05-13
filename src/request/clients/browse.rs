use crate::{
    models::fast_browse::BrowseResponse,
    request::clients::{
        endpoints::BROWSE_ENDPOINT,
        shared_params::{CLIENT_NAME_HEADER_NAME, CLIENT_VERSION_HEADER_NAME, CONTENT_TYPE_HEADER, ORIGIN_HEADER, USER_AGENT_HEADER_NAME},
    },
    shared_traits::{ClientPrebuild, ClientWithHeaders},
};

const USER_AGENT: &str = "User-Agent: Mozilla/5.0 (Linux; Android 10; Quest 2) AppleWebKit/537.36 (KHTML, like Gecko) OculusBrowser/32.0.0.3.65 SamsungBrowser/4.3 Chrome/137.0.7151.61 Mobile VR Safari/537.36";
const CLIENT_NAME: &str = "28";
const CLIENT_VERSION: &str = "1.60.19";

pub struct BrowseClient {}

impl ClientWithHeaders for BrowseClient {
    type Response = BrowseResponse;

    fn get_client() -> ClientPrebuild {
        let client = reqwest::Client::new();

        client
            .post(BROWSE_ENDPOINT)
            .header(CONTENT_TYPE_HEADER.0, CONTENT_TYPE_HEADER.1)
            .header(USER_AGENT_HEADER_NAME, USER_AGENT)
            .header(CLIENT_NAME_HEADER_NAME, CLIENT_NAME)
            .header(CLIENT_VERSION_HEADER_NAME, CLIENT_VERSION)
            .header(ORIGIN_HEADER.0, ORIGIN_HEADER.1)
    }
}
