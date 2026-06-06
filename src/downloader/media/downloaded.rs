use std::{collections::HashMap, fmt::Debug, path::Path};

use bytes::BytesMut;

use crate::{
    Result,
    downloader::streams::{AnyStream, MediaStream, Thumbnail},
    error::YtuwuError,
    metadata::MediaMetadata,
    name_trimmer::default_trim,
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

    fn to_any(self) -> AnyStream {
        self.stream.to_any()
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
    pub fn new(streams: Vec<AnyStream>, metadata: MediaMetadata, thumbnail: Option<Thumbnail>) -> Self {
        Self { thumbnail, streams, metadata }
    }

    pub fn from_dwn_media<M1, M2>(media_one: DwnMedia<M1>, media_two: DwnMedia<M2>) -> Self
    where
        M1: MediaStream + Debug,
        M2: MediaStream + Debug,
    {
        let DwnMedia {
            thumbnail: thumb_one,
            stream: stream_one,
            ..
        } = media_one;

        let DwnMedia {
            thumbnail: thumb_two,
            stream: stream_two,
            metadata,
        } = media_two;

        let thumbnail = thumb_one.or(thumb_two);
        let stream_one = stream_one.to_any();
        let stream_two = stream_two.to_any();

        Self {
            streams: vec![stream_one, stream_two],
            thumbnail,
            metadata,
        }
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
