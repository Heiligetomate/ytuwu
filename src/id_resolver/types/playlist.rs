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

        let id_len = raw_id.len();
        let pl_start = raw_id.starts_with("PL");
        let rd_start = raw_id.starts_with("RDCLAK5uy");

        if id_len == 43 && !rd_start {
            return Err(YtuwuError::InvalidIdFormat(("PlaylistId", "RDCLAK5uy*")));
        } else if id_len != 43 && rd_start {
            return Err(YtuwuError::InvalidIdLength(("PlaylistId", 43)));
        } else if id_len == 34 && !pl_start {
            return Err(YtuwuError::InvalidIdFormat(("PlaylistId", "PL*")));
        } else if id_len != 34 && pl_start {
            return Err(YtuwuError::InvalidIdLength(("PlaylistId", 34)));
        } else if id_len != 34 && !pl_start && !rd_start {
            return Err(YtuwuError::InvalidIdFormat(("PlaylistId", "RDCLAK5uy* / Pl*")));
        }

        Ok(Self { id: format!("VL{}", raw_id) })
    }

    fn get_id(self) -> String {
        self.id
    }

    fn as_str(&self) -> &str {
        &self.id
    }
}
