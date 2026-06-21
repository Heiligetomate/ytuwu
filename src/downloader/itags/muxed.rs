use serde::{Deserialize, Serialize};

use crate::{
    Result,
    downloader::{itags::core::Itag, mime_types::MimeType, streams::MuxedStream},
    error::YtuwuError,
    itags::AnyItag,
};

/// MuxedItag contains all muxed formats.
/// It also contains a Highest variant which should be used if the stream quality should be.
/// downgraded to avoid non existent streams.
/// This is useless here because it only has one variant.
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
