use std::sync::Arc;

use crate::{
    Result,
    downloader::{
        Downloader,
        builders::{channel::EmptyChannelBuilder, media::EmptyMediaBuilder, playlist::EmptyListBuilder},
    },
    id_resolver::IdCollection,
};

/// Contains an arc to the downloader for shared data and an IdCollection that stores all extracted
/// ids
#[derive(Debug)]
pub struct EmptyBuilder {
    pub downloader: Arc<Downloader>,
    pub ids: IdCollection,
}

impl EmptyBuilder {
    /// Creates a new EmptyBuilder with the given values
    pub fn new(downloader: Arc<Downloader>, ids: IdCollection) -> Self {
        Self { downloader, ids }
    }

    /// Consumes itself and converts it into an EmptyMediaBuilder for downloading single tracks
    pub fn as_media(self) -> Result<EmptyMediaBuilder> {
        EmptyMediaBuilder::new(self)
    }

    /// Consumes itself and converts it into an EmptyListBuilder for downloading albums or playlists
    pub fn as_list(self) -> Result<EmptyListBuilder> {
        EmptyListBuilder::new(self)
    }

    /// Consumes itself and converts it into an EmptyChannelBuilder for downloading entire channels or artists
    pub fn as_channel(self) -> Result<EmptyChannelBuilder> {
        EmptyChannelBuilder::new(self)
    }
}
