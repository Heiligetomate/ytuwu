use std::path::Path;

use crate::{
    downloader::{
        mime_types::MimeType,
        streams::{MediaStream, util::save_media_stream},
    },
    error::Result,
};
use bytes::{BufMut, Bytes, BytesMut};

/// Holds data as Bytes
/// Implements MediaStream
#[derive(Debug, PartialEq, Eq)]
pub struct Thumbnail {
    data: BytesMut,
}

impl MediaStream for Thumbnail {
    fn get_mime_type(&self) -> MimeType {
        MimeType::Png
    }

    fn get_data(&self) -> &BytesMut {
        &self.data
    }

    fn push_data(&mut self, data: Bytes) {
        self.data.put(data);
    }

    fn save(&self, path: &Path, file_name: &str) -> Result<()> {
        save_media_stream(path, file_name, self)?;
        Ok(())
    }
}

impl Thumbnail {
    /// Creates a new empty Thumbnail stream
    pub fn new() -> Self {
        Self { data: BytesMut::new() }
    }
}
