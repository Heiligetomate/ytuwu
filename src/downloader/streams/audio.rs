use std::path::Path;

use bytes::{BufMut, Bytes, BytesMut};

use crate::{
    Result,
    downloader::{
        mime_types::MimeType,
        streams::{AnyStream, core::MediaStream, util::save_media_stream},
    },
    itags::{AudioItag, Itag},
};

/// Holds data as Bytes and mime type
/// Gets created when AudioIta gets used
/// Implements MediaStream
#[derive(Debug, PartialEq, Eq)]
pub struct AudioStream {
    data: BytesMut,
    mime_type: MimeType,
}

impl MediaStream for AudioStream {
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

impl From<AudioStream> for AnyStream {
    fn from(s: AudioStream) -> Self {
        AnyStream::Audio(s)
    }
}

impl AudioStream {
    /// Takes an AudioItag and creates a new AudioStream from that
    pub fn new(itag: AudioItag) -> Self {
        Self {
            data: BytesMut::new(),
            mime_type: itag.get_mime_type(),
        }
    }
}
