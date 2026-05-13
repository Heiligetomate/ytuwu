use crate::models::channel_browse::{ChannelBrowse, ChannelBrowseResponse};
use crate::models::fast_browse::BrowseResponse;
use crate::models::player::PlayerResponse;
use crate::request::{self, parameters::*};
use crate::{request::parameters::BROWSE_ENDPOINT, shared_traits::Response};
use reqwest::RequestBuilder;

type ClientPrebuild = RequestBuilder;

pub trait ClientWithHeaders {
    type Response: Response;
    fn get_prebuild(&self) -> ClientPrebuild;
}

pub struct BrowseClient {}

pub struct PlayerClient {}

pub struct ChannelClient {}

impl ClientWithHeaders for BrowseClient {
    type Response = BrowseResponse;

    fn get_prebuild(&self) -> ClientPrebuild {
        let client = reqwest::Client::new();

        client
            .post(BROWSE_ENDPOINT)
            .header(CONTENT_TYPE_HEADER.0, CONTENT_TYPE_HEADER.1)
            .header(USER_AGENT_HEADER.0, USER_AGENT_HEADER.1)
            .header(CLIENT_NAME_HEADER.0, CLIENT_NAME_HEADER.1)
            .header(CLIENT_VERSION_HEADER.0, CLIENT_VERSION_HEADER.1)
            .header(ORIGIN_HEADER.0, ORIGIN_HEADER.1)
    }
}

impl ClientWithHeaders for PlayerClient {
    type Response = PlayerResponse;

    fn get_prebuild(&self) -> ClientPrebuild {
        let client = reqwest::Client::new();

        client
            .post(PLAYER_ENDPOINT)
            .header(CONTENT_TYPE_HEADER.0, CONTENT_TYPE_HEADER.1)
            .header(USER_AGENT_HEADER.0, USER_AGENT_HEADER.1)
            .header(CLIENT_NAME_HEADER.0, CLIENT_NAME_HEADER.1)
            .header(CLIENT_VERSION_HEADER.0, CLIENT_VERSION_HEADER.1)
            .header(ORIGIN_HEADER.0, ORIGIN_HEADER.1)
    }
}

impl ClientWithHeaders for ChannelClient {
    type Response = ChannelBrowseResponse;

    fn get_prebuild(&self) -> ClientPrebuild {
        let client = reqwest::Client::new();
    }
}
