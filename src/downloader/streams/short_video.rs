use std::path::Path;

use bytes::{BufMut, Bytes, BytesMut};

use crate::{
    Result,
    downloader::{
        mime_types::MimeType,
        streams::{
            core::{MediaStream, VideoStream},
            util::save_media_stream,
        },
    },
    itags::{Itag, ShortItag},
    streams::AnyStream,
};

#[derive(Debug, PartialEq, Eq)]
pub struct ShortVideoStream {
    data: BytesMut,
    mime_type: MimeType,
}

impl VideoStream for ShortVideoStream {}

impl MediaStream for ShortVideoStream {
    fn to_any(self) -> AnyStream {
        AnyStream::ShortVideo(self)
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

impl ShortVideoStream {
    pub fn new(itag: ShortItag) -> Self {
        Self {
            data: BytesMut::new(),
            mime_type: itag.get_mime_type(),
        }
    }
}
