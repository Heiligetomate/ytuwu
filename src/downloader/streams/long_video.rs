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
    itags::{Itag, VideoItag},
    streams::AnyStream,
};

#[derive(Debug)]
pub struct LongVideoStream {
    data: BytesMut,
    mime_type: MimeType,
}

impl VideoStream for LongVideoStream {}

impl MediaStream for LongVideoStream {
    fn to_any(self) -> AnyStream {
        AnyStream::LongVideo(self)
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

impl LongVideoStream {
    pub fn new(itag: VideoItag) -> Self {
        Self {
            data: BytesMut::new(),
            mime_type: itag.get_mime_type(),
        }
    }
}
