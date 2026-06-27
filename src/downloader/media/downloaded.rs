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

/// This struct gets created when calling download on a Media object
/// Holds exactly one stream, metadata, an uuid for identification and a thumbnail if one was
/// downloaded
/// This struct is generic because it has to hold the mediastream
#[derive(Debug)]
pub struct DwnMedia<M: MediaStream + Debug> {
    pub metadata: MediaMetadata,
    pub stream: M,
    pub thumbnail: Option<Thumbnail>,
    pub id: Uuid,
}

/// This struct gets created when calling download_bundle on a Media object.
/// Holds all downloaded streams (can be infinite theoratically), metadata, an uuid for
/// identification and a thumbnail if one was downloaded.
/// This struct is not generict because it holds AnyStream which is a wrapper for all mediastreams
#[derive(Debug)]
pub struct DwnBundleMedia {
    pub metadata: MediaMetadata,
    pub streams: Vec<AnyStream>,
    pub thumbnail: Option<Thumbnail>,
    pub id: Uuid,
}

impl<M: MediaStream + Debug> DwnMedia<M> {
    /// Creates a new DwnMedia with the given parameters.
    pub fn new(stream: M, metadata: MediaMetadata, thumbnail: Option<Thumbnail>, id: Uuid) -> Self {
        Self { metadata, stream, thumbnail, id }
    }

    /// Consumes self and converts the mediastream inside itself into an anystream which is not
    /// generic anymore
    /// All other fields are used again for the new object
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

    /// Returns a reference to the innser data of self as BytesMut
    pub fn bytes(&self) -> &BytesMut {
        self.stream.get_data()
    }

    /// Returns a reference to the inner thumbnail
    /// Fails if there was no thumbnail downloaded and therefore no thumbnail exists
    pub fn get_thumbnail(&self) -> Result<&Thumbnail> {
        self.thumbnail
            .as_ref()
            .ok_or(YtuwuError::NoThumbnail(self.metadata.title.clone()))
    }

    /// saves the media stream by trimming the name and calling .save on the mediastream by using
    /// the timmed name as file name and the given path as Path
    /// Returns Err if the saving went wrong
    pub fn save_media_stream(&self, path: &Path) -> Result<()> {
        self.stream
            .save(path, default_trim(&self.metadata.title).as_str())
    }

    /// saves the thumnail by trimming the name and calling .save on the thumnail by using
    /// the timmed name as file name and the given path as Path
    /// Returns Err if the saving went wrong or the thumnail does not exist because it was never
    /// downloaded
    pub fn save_thumbnail(&self, path: &Path) -> Result<()> {
        self.get_thumbnail()?
            .save(path, default_trim(&self.metadata.title).as_str())?;
        Ok(())
    }

    /// Saves both the thumnail and the media stream with the given path
    /// Fails if either of those functions fail
    pub fn save_full(&self, path: &Path) -> Result<()> {
        self.save_thumbnail(&path)?;
        self.save_media_stream(&path)?;
        Ok(())
    }

    /// Returns a reference to the bytes of the thumbnail as BytesMut
    /// Fails if there is no thumbnail downloaded and stored in DwnMedia
    pub fn thumbnail_bytes(&self) -> Result<&BytesMut> {
        Ok(self.get_thumbnail()?.get_data())
    }
}

impl DwnBundleMedia {
    /// Creates a new DwnBundleMedia with the parameters
    /// The streams have to be AnyStream.
    /// Call .into() on any Mediastream to convert it to anystream.
    pub fn new(streams: Vec<AnyStream>, metadata: MediaMetadata, thumbnail: Option<Thumbnail>, id: Uuid) -> Self {
        Self { thumbnail, streams, metadata, id }
    }

    /// Takes a vec of DwnMedia with Anystream as stream and converts them to a DwnBundleMedia
    /// This is achieved by getting the metadata from the first field, get the first thumbnail that
    /// exists in the vec of medias and merging all mediastreams of the media to one bundle.
    /// For the id, it also takes the id of the first song in the list, however this has to be
    /// changed
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

    /// Returns a reference to the inner thumbnail
    /// Returns Err if there was no thumbnail downloaded and therefore no thumbnail exists
    pub fn get_thumbnail(&self) -> Result<&Thumbnail> {
        self.thumbnail
            .as_ref()
            .ok_or(YtuwuError::NoThumbnail(self.metadata.title.clone()))
    }

    /// saves the thumnail by trimming the name and calling .save on the thumnail by using
    /// the timmed name as file name and the given path as Path
    /// Returns Err if the saving went wrong or the thumnail does not exist because it was never
    /// downloaded
    pub fn save_thumbnail(&self, path: &Path) -> Result<()> {
        self.get_thumbnail()?
            .save(path, default_trim(&self.metadata.title).as_str())
    }

    // TODO: is .into_dyn needed here? this should be avoidable by using anystreams
    /// Saves all media streams.
    /// iterates over every stream contained.
    /// Saves every stream with the trimmed file name and the given path, the mime type is
    /// automatically getting attached. When there are streams that have the same mimetype, there
    /// would be the same stream twice which is why this tracks already saved files and adds an
    /// index if the file name was already used.
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

    /// Saves both the thumnail and the media stream with the given path
    /// Fails if either of those functions fail
    pub fn save_full(&self, path: &Path) -> Result<()> {
        self.save_thumbnail(&path)?;
        self.save_media_streams(&path)?;
        Ok(())
    }

    /// Returns a reference to the bytes of the thumbnail as BytesMut
    /// Fails if there is no thumbnail downloaded and stored in DwnMedia
    pub fn thumbnail_bytes(&self) -> Result<&BytesMut> {
        Ok(self.get_thumbnail()?.get_data())
    }
}
