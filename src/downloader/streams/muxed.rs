use std::path::Path;

use bytes::{BufMut, Bytes, BytesMut};

use crate::{
    Result,
    downloader::{
        mime_types::MimeType,
        streams::{AnyStream, core::MediaStream, util::save_media_stream},
    },
    itags::{Itag, MuxedItag},
};

#[derive(Debug, PartialEq, Eq)]
pub struct MuxedStream {
    data: BytesMut,
    mime_type: MimeType,
}

impl MediaStream for MuxedStream {
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

impl From<MuxedStream> for AnyStream {
    fn from(value: MuxedStream) -> Self {
        AnyStream::Muxed(value)
    }
}

impl MuxedStream {
    pub fn new(itag: MuxedItag) -> Self {
        Self {
            data: BytesMut::new(),
            mime_type: itag.get_mime_type(),
        }
    }
}
