use std::path::Path;

use bytes::{BufMut, Bytes, BytesMut};

use crate::{
    Result,
    downloader::{
        mime_types::MimeType,
        streams::{core::MediaStream, util::save_media_stream},
    },
    itags::{AudioItag, Itag},
    streams::AnyStream,
};

#[derive(Debug, PartialEq, Eq)]
pub struct AudioStream {
    data: BytesMut,
    mime_type: MimeType,
}

impl MediaStream for AudioStream {
    fn to_any(self) -> AnyStream {
        AnyStream::Audio(self)
    }

    fn get_data(&self) -> &BytesMut {
        &self.data
    }

    fn save(&self, path: &Path, file_name: &str) -> Result<()> {
        save_media_stream(path, &file_name, self)?;
        Ok(())
    }

    fn push_data(&mut self, data: Bytes) {
        self.data.put(data);
    }

    fn get_mime_type(&self) -> MimeType {
        self.mime_type
    }
}

impl AudioStream {
    pub fn new(itag: AudioItag) -> Self {
        Self {
            data: BytesMut::new(),
            mime_type: itag.get_mime_type(),
        }
    }
}
