use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    Result,
    downloader::{itags::core::Itag, mime_types::MimeType, streams::ShortStream},
    error::YtuwuError,
};

/// ShortItag contains all pure short video formats.
/// It also contains a Highest variant which should be used if the stream quality should be.
/// downgraded to avoid non existent streams.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum ShortItag {
    Highest,
    Low,  // 779
    High, // 780
}

/// Order containing every ShortItag variant
/// Used for next best
const SHORT_LONG_VIDEO_ORDER: [ShortItag; 3] = [ShortItag::Highest, ShortItag::High, ShortItag::Low];

impl Itag for ShortItag {
    type Stream = ShortStream;

    fn highest() -> Self {
        Self::High
    }

    fn is_highest(&self) -> bool {
        *self == Self::Highest
    }

    fn next_best(self) -> Result<Self>
    where
        Self: Sized,
    {
        for (i, itag) in SHORT_LONG_VIDEO_ORDER
            .iter()
            .enumerate()
        {
            if *itag == self {
                let next_itag = SHORT_LONG_VIDEO_ORDER
                    .get(i + 1)
                    .ok_or(YtuwuError::NoLowerItagFound)?;
                return Ok(*next_itag);
            }
        }
        panic!("Itag doesnt exit")
    }

    fn to_int(&self) -> u16 {
        match &self {
            Self::Highest => Self::highest().to_int(),
            Self::Low => 779,
            Self::High => 780,
        }
    }

    fn get_mime_type(&self) -> MimeType {
        MimeType::MP4
    }

    fn new_stream(self) -> Self::Stream {
        ShortStream::new(self)
    }

    fn to_any(self) -> super::AnyItag {
        super::AnyItag::Short(self)
    }
}

impl Display for ShortItag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw = match self {
            ShortItag::Highest => "Highest",
            ShortItag::Low => "Low",
            ShortItag::High => "High",
        };
        write!(f, "Short video: {}", raw)
    }
}
