use std::path::Path;

use crate::{
    downloader::mime_types::MimeType,
    error::Result,
    streams::{AnyStream, MediaStream, util::save_media_stream},
};
use bytes::{BufMut, Bytes, BytesMut};

#[derive(Debug)]
pub struct Thumbnail {
    data: BytesMut,
}

impl MediaStream for Thumbnail {
    fn to_any(self) -> AnyStream {
        AnyStream::Thumbnail(self)
    }

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
    pub fn new(data: Bytes) -> Self {
        Self { data: data.into() }
    }
}
