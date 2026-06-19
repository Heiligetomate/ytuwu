use std::{collections::HashMap, fmt::Debug, path::Path};

use bytes::BytesMut;
use uuid::Uuid;

use crate::{
    Result,
    downloader::{
        metadata::MediaMetadata,
        streams::{AnyStream, MediaStream, Thumbnail},
    },
    error::YtuwuError,
    name_trimmer::default_trim,
};

#[derive(Debug)]
pub struct DwnMedia<M: MediaStream + Debug> {
    pub metadata: MediaMetadata,
    pub stream: M,
    pub thumbnail: Option<Thumbnail>,
    pub id: Uuid,
}

#[derive(Debug)]
pub struct DwnBundleMedia {
    pub metadata: MediaMetadata,
    pub streams: Vec<AnyStream>,
    pub thumbnail: Option<Thumbnail>,
    pub id: Uuid,
}

impl<M: MediaStream + Debug> DwnMedia<M> {
    pub fn new(stream: M, metadata: MediaMetadata, thumbnail: Option<Thumbnail>, id: Uuid) -> Self {
        Self { metadata, stream, thumbnail, id }
    }

    pub fn to_any(self) -> DwnMedia<AnyStream>
    where
        M: Into<AnyStream>,
    {
        DwnMedia {
            metadata: self.metadata,
            stream: self.stream.into(),
            thumbnail: self.thumbnail,
            id: self.id,
        }
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
            .save(path, default_trim(&self.metadata.title).as_str())
    }

    pub fn save_thumbnail(&self, path: &Path) -> Result<()> {
        self.get_thumbnail()?
            .save(path, default_trim(&self.metadata.title).as_str())?;
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
    pub fn new(streams: Vec<AnyStream>, metadata: MediaMetadata, thumbnail: Option<Thumbnail>, id: Uuid) -> Self {
        Self { thumbnail, streams, metadata, id }
    }

    pub fn from_dwn_medias(medias: Vec<DwnMedia<AnyStream>>) -> Result<Self> {
        let mut iter = medias.into_iter();
        let first = iter
            .next()
            .ok_or(YtuwuError::EmptyMediaBundle)?;
        let metadata = first.metadata;
        let mut thumbnail = first.thumbnail;
        let mut streams = vec![first.stream];

        for media in iter {
            if thumbnail.is_none() {
                thumbnail = media.thumbnail;
            }
            streams.push(media.stream);
        }

        Ok(Self {
            streams,
            thumbnail,
            metadata,
            id: first.id, // TODO: not good
        })
    }

    pub fn get_thumbnail(&self) -> Result<&Thumbnail> {
        self.thumbnail
            .as_ref()
            .ok_or(YtuwuError::NoThumbnail)
    }

    pub fn save_thumbnail(&self, path: &Path) -> Result<()> {
        self.get_thumbnail()?
            .save(path, default_trim(&self.metadata.title).as_str())
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
            let title = default_trim(&self.metadata.title);
            let name = if *count == 0 { title } else { format!("{}-{}", title, count) };
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
