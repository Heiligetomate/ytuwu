use serde::{Deserialize, Serialize};

use crate::{
    Result,
    downloader::{itags::core::Itag, mime_types::MimeType},
    error::YtuwuError,
    itags::AnyItag,
    streams::MuxedStream,
};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum MuxedItag {
    Highest,
    MuxedMP4,
}

impl Itag for MuxedItag {
    type Stream = MuxedStream;

    fn highest() -> Self {
        Self::MuxedMP4
    }

    fn is_highest(&self) -> bool {
        *self == Self::Highest
    }

    fn to_int(&self) -> u16 {
        18
    }

    fn next_best(self) -> Result<Self> {
        Err(YtuwuError::NoLowerItagFound)
    }

    fn get_mime_type(&self) -> MimeType {
        MimeType::MP4
    }

    fn new_stream(self) -> Self::Stream {
        MuxedStream::new(self)
    }

    fn to_any(self) -> super::AnyItag {
        AnyItag::Muxed(self)
    }
}
