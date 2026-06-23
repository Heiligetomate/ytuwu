use serde::{Deserialize, Serialize};

use crate::{Result, error::YtuwuError, id_resolver::id::Id, request::clients::SlowBrowseClient};

/// This struct is for the content that gets extracted when browsing a channel
/// The if format is different to the other browse ids
/// When creating this, ther len should be 17 and the id should start with "MPREb_"
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
            return Err(YtuwuError::InvalidIdLength(("ChannelPlaylistId", 17)));
        }

        if !raw_id.starts_with("MPREb_") {
            return Err(YtuwuError::InvalidIdFormat);
        }

        Ok(Self { id: raw_id })
    }
}
