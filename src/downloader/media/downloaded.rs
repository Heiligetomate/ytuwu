use std::{fmt::Debug, path::Path};

use bytes::{Bytes, BytesMut};

use crate::{
    Result,
    downloader::{
        media_stream::{AnyStream, MediaStream},
        thumbnail::Thumbnail,
    },
    metadata::MediaMetadata,
};

#[derive(Debug)]
pub struct DownloadedMediaWithThumbnail<M: MediaStream + Debug> {
    pub metadata: MediaMetadata,
    pub thumbnail: Thumbnail,
    pub stream: M,
}

#[derive(Debug)]
pub struct DownloadedMedia<M: MediaStream + Debug> {
    pub metadata: MediaMetadata,
    pub stream: M,
}

#[derive(Debug)]
pub struct MultipleStreamMedia {
    pub metadata: MediaMetadata,
    pub streams: Vec<AnyStream>,
}

impl<M: MediaStream + Debug> DownloadedMedia<M> {
    pub fn new(stream: M, metadata: MediaMetadata) -> Self {
        Self { metadata, stream }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        self.stream
            .save(path, &self.metadata.title)
    }

    pub fn bytes(&self) -> &BytesMut {
        self.stream.get_data()
    }
}

impl<M: MediaStream + Debug> DownloadedMediaWithThumbnail<M> {
    pub fn new(stream: M, thumbnail: Thumbnail, metadata: MediaMetadata) -> Self {
        Self { thumbnail, stream, metadata }
    }

    pub fn save_thumbnail(&self, path: &Path) -> Result<()> {
        self.thumbnail.save(path)?;
        Ok(())
    }

    pub fn save_media_stream(&self, path: &Path) -> Result<()> {
        self.stream
            .save(path, &self.metadata.title)?;
        Ok(())
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        self.save_thumbnail(&path)?;
        self.save_media_stream(&path)?;
        Ok(())
    }

    pub fn thumbnail_bytes(&self) -> &Bytes {
        self.thumbnail.bytes()
    }

    pub fn bytes(&self) -> &BytesMut {
        self.stream.get_data()
    }
}
