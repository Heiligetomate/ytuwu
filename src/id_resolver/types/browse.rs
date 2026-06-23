use serde::{Deserialize, Serialize};

use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{GetId, Id, IdCollection},
    request::clients::DummyClient,
    types::{AlbumId, ChannelPlaylistId, PlaylistId},
};

/// This is a wrapper for all Browse ids which is needed for a clean .get_id() in the api
/// When creating a new BrowseId, it tries to create all different browse ids
/// This works because all Browse ids have a different format
/// All other functions just match self and return the function call of the matched value
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum BrowseId {
    PlaylistId(PlaylistId),
    ChannelBrowseId(ChannelPlaylistId),
    AlbumId(AlbumId),
}

impl Id for BrowseId {
    type Client = DummyClient;

    fn new<T: Into<String>>(id: T) -> crate::Result<Self> {
        let raw = id.into();

        if let Ok(id) = PlaylistId::new(&raw) {
            return Ok(Self::PlaylistId(id));
        } else if let Ok(id) = AlbumId::new(&raw) {
            return Ok(Self::AlbumId(id));
        } else if let Ok(id) = ChannelPlaylistId::new(&raw) {
            return Ok(Self::ChannelBrowseId(id));
        }
        return Err(YtuwuError::IdCreationError(Some("Creating a browse id from the raw id was not possible because no format matched".into())));
    }

    fn get_id(self) -> String {
        match self {
            Self::PlaylistId(id) => id.get_id(),
            Self::AlbumId(id) => id.get_id(),
            Self::ChannelBrowseId(id) => id.get_id(),
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Self::PlaylistId(id) => id.as_str(),
            Self::AlbumId(id) => id.as_str(),
            Self::ChannelBrowseId(id) => id.as_str(),
        }
    }
}

impl GetId<BrowseId> for IdCollection {
    fn get_id(&self) -> Result<BrowseId> {
        Ok(self
            .browse_id
            .clone()
            .ok_or(crate::error::get_id_err("browseId", &self))?)
    }
}
