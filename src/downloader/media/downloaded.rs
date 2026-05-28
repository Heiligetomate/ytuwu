use std::{collections::HashMap, fmt::Debug, path::Path};

use bytes::BytesMut;

use crate::{
    Result,
    downloader::streams::{AnyStream, MediaStream, Thumbnail},
    error::YtuwuError,
    metadata::MediaMetadata,
};

#[derive(Debug)]
pub struct DwnMedia<M: MediaStream + Debug> {
    pub metadata: MediaMetadata,
    pub stream: M,
    pub thumbnail: Option<Thumbnail>,
}

#[derive(Debug)]
pub struct DwnBundleMedia {
    pub metadata: MediaMetadata,
    pub streams: Vec<AnyStream>,
    pub thumbnail: Option<Thumbnail>,
}

impl<M: MediaStream + Debug> DwnMedia<M> {
    pub fn new(stream: M, metadata: MediaMetadata, thumbnail: Option<Thumbnail>) -> Self {
        Self { metadata, stream, thumbnail }
    }

    pub fn bytes(&self) -> &BytesMut {
        self.stream.get_data()
    }

    pub fn get_thumbnail(&self) -> Result<&Thumbnail> {
        self.thumbnail
            .as_ref()
            .ok_or(YtuwuError::NoThumbnail)
    }

    pub fn save_media_stream(&self, path: &Path) -> Result<()> {
        self.stream
            .save(path, &self.metadata.title)
    }

    pub fn save_thumbnail(&self, path: &Path) -> Result<()> {
        self.get_thumbnail()?
            .save(path, &self.metadata.title)?;
        Ok(())
    }

    pub fn save_full(&self, path: &Path) -> Result<()> {
        self.save_thumbnail(&path)?;
        self.save_media_stream(&path)?;
        Ok(())
    }

    pub fn thumbnail_bytes(&self) -> Result<&BytesMut> {
        Ok(self.get_thumbnail()?.get_data())
    }
}

impl DwnBundleMedia {
    pub fn new(streams: Vec<AnyStream>, metadata: MediaMetadata, thumbnail: Option<Thumbnail>) -> Self {
        Self { thumbnail, streams, metadata }
    }

    pub fn get_thumbnail(&self) -> Result<&Thumbnail> {
        self.thumbnail
            .as_ref()
            .ok_or(YtuwuError::NoThumbnail)
    }

    pub fn save_thumbnail(&self, path: &Path) -> Result<()> {
        self.get_thumbnail()?
            .save(path, &self.metadata.title)
    }

    pub fn save_media_streams(&self, path: &Path) -> Result<()> {
        let mut seen: HashMap<String, u32> = HashMap::new();
        for stream in self.streams.iter() {
            let dyn_stream = stream.into_dyn();
            let mime_type = dyn_stream
                .get_mime_type()
                .as_str()
                .to_owned();
            let count = seen.entry(mime_type).or_insert(0);
            let name = if *count == 0 { self.metadata.title.clone() } else { format!("{}-{}", self.metadata.title, count) };
            *count += 1;
            dyn_stream.save(path, &name)?;
        }
        Ok(())
    }

    pub fn save_full(&self, path: &Path) -> Result<()> {
        self.save_thumbnail(&path)?;
        self.save_media_streams(&path)?;
        Ok(())
    }

    pub fn thumbnail_bytes(&self) -> Result<&BytesMut> {
        Ok(self.get_thumbnail()?.get_data())
    }
}
