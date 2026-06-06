use std::path::Path;

use bytes::{BufMut, Bytes, BytesMut};

use crate::{
    Result,
    downloader::{
        mime_types::MimeType,
        streams::{core::MediaStream, util::save_media_stream},
    },
    itags::{Itag, VideoItag},
    streams::AnyStream,
};

#[derive(Debug, PartialEq, Eq)]
pub struct VideoStream {
    data: BytesMut,
    mime_type: MimeType,
}

impl MediaStream for VideoStream {
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

impl From<VideoStream> for AnyStream {
    fn from(s: VideoStream) -> Self {
        AnyStream::Video(s)
    }
}

impl VideoStream {
    pub fn new(itag: VideoItag) -> Self {
        Self {
            data: BytesMut::new(),
            mime_type: itag.get_mime_type(),
        }
    }
}
