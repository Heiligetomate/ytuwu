use crate::{
    Id,
    error::YtuwuError,
    request::clients::dummy::DummyClient,
    types::{AlbumId, ChannelPlaylistId, PlaylistId},
};

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
        return Err(YtuwuError::NoIdFound);
    }

    fn get_id(self) -> String {
        self.get_id().to_owned()
    }

    fn as_str(&self) -> &str {
        match self {
            Self::PlaylistId(id) => id.as_str(),
            Self::AlbumId(id) => id.as_str(),
            Self::ChannelBrowseId(id) => id.as_str(),
        }
    }
}
