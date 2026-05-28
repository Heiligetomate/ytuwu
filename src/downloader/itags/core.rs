use crate::{Result, downloader::mime_types::MimeType, streams::MediaStream};

pub trait Itag {
    type Stream: MediaStream;
    fn is_highest(&self) -> bool;
    fn highest() -> Self
    where
        Self: Sized;
    fn next_best(self) -> Result<Self>
    where
        Self: Sized;

    fn to_int(&self) -> u16;
    fn get_mime_type(&self) -> MimeType;

    fn new_stream(self) -> Self::Stream;
}
