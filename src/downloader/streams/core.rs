use std::path::Path;

use bytes::{Bytes, BytesMut};

use crate::{Result, downloader::mime_types::MimeType, streams::AnyStream};

pub trait MediaStream {
    fn to_any(self) -> AnyStream;
    fn get_mime_type(&self) -> MimeType;
    fn save(&self, path: &Path, file_name: &str) -> Result<()>;
    fn get_data(&self) -> &BytesMut;
    fn push_data(&mut self, data: Bytes);
}

pub trait VideoStream: MediaStream {}
