use serde::{Deserialize, Serialize};

use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{browse_id::BrowseId, id::Id},
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

    fn new<T: Into<String>>(id: T) -> Result<Self> {
        let raw_id = id.into();

        if raw_id.len() != 17 {
            return Err(YtuwuError::InvalidIdLength);
        }

        if !raw_id.starts_with("MPREb_") {
            return Err(YtuwuError::InvalidIdFormat);
        }

        Ok(Self { id: raw_id })
    }
}

impl BrowseId for ChannelPlaylistId {
    type BrowseResponse = SlowBrowseResponse;
}
