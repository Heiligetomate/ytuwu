use serde::{Deserialize, Serialize};

use crate::{
    id_resolver::id::{BrowseId, Id},
    models::slow_browse::SlowBrowseResponse,
    request::clients::slow_browse::SlowBrowseClient,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ChannelPlaylistId {
    id: String,
}

impl Id for ChannelPlaylistId {
    type Client = SlowBrowseClient;

    fn get_id(self) -> String {
        self.id
    }

    fn as_str(&self) -> &str {
        &self.id
    }

    fn new<T: Into<String>>(id: T) -> Self {
        Self { id: id.into() }
    }
}

impl BrowseId for ChannelPlaylistId {
    type BrowseResponse = SlowBrowseResponse;
}
