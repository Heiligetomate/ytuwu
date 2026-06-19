use std::sync::Arc;

use crate::{
    Result,
    downloader::{
        Downloader,
        builders::{channel::EmptyChannelBuilder, media::EmptyMediaBuilder, playlist::EmptyListBuilder},
    },
    id_resolver::IdCollection,
};

#[derive(Debug)]
pub struct EmptyBuilder {
    pub downloader: Arc<Downloader>,
    pub ids: IdCollection,
}

impl EmptyBuilder {
    pub fn new(downloader: Arc<Downloader>, ids: IdCollection) -> Self {
        Self { downloader, ids }
    }

    pub fn as_media(self) -> Result<EmptyMediaBuilder> {
        EmptyMediaBuilder::new(self)
    }

    pub fn as_list(self) -> Result<EmptyListBuilder> {
        EmptyListBuilder::new(self)
    }

    pub fn as_channel(self) -> Result<EmptyChannelBuilder> {
        EmptyChannelBuilder::new(self)
    }
}
