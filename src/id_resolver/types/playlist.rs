use crate::{Result, error::YtuwuError, id_resolver::id::Id, request::clients::PlaylistBrowseClient};

use serde::{Deserialize, Serialize};

/// This id is used for browsing playlists, not albums or channel browsed stuff
/// When crating a new PlaylistId, i should either start with RDCLAK5uy and have a len of 43 or it should start with PL and have a len of 34.
/// "VL" is added in front to both of them after they are validated
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PlaylistId {
    id: String,
}

impl Id for PlaylistId {
    type Client = PlaylistBrowseClient;

    fn new<T: Into<String>>(id: T) -> Result<Self> {
        let raw_id = id.into();

        // TODO: Better handling so the id length gets the correct len error
        match raw_id.len() {
            43 => {
                if !raw_id.starts_with("RDCLAK5uy") {
                    return Err(YtuwuError::InvalidIdFormat(("PlaylistId", "RDCLAK5uy*")));
                }
            }
            34 => {
                if !raw_id.starts_with("PL") {
                    return Err(YtuwuError::InvalidIdFormat(("PlaylistId", "PL*")));
                }
            }
            _ => return Err(YtuwuError::InvalidIdLength(("PlaylistId", 34))),
        };

        Ok(Self { id: format!("VL{}", raw_id) })
    }

    fn get_id(self) -> String {
        self.id
    }

    fn as_str(&self) -> &str {
        &self.id
    }
}
