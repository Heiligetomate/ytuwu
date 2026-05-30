use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{
        browse_id::BrowseId,
        collection::IdCollection,
        id::{GetId, Id},
    },
    models::playlist::PlaylistResponse,
    request::clients::playlist::PlaylistBrowseClient,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PlaylistId {
    id: String,
}

impl Id for PlaylistId {
    type Client = PlaylistBrowseClient;

    fn new<T: Into<String>>(id: T) -> Result<Self> {
        let raw_id = id.into();

        if raw_id.len() != 43 {
            return Err(YtuwuError::InvalidIdLength);
        }

        if !raw_id.starts_with("RDCLAK5uy") {
            return Err(YtuwuError::InvalidIdFormat);
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

impl GetId<PlaylistId> for IdCollection {
    fn get_id(&self) -> Result<PlaylistId> {
        Ok(self
            .playlist_id
            .clone()
            .ok_or(YtuwuError::NoIdFound)?)
    }
}

impl BrowseId for PlaylistId {
    type BrowseResponse = PlaylistResponse;
}
