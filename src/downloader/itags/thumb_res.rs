use std::fmt::Display;

use crate::{
    downloader::{mime_types::MimeType, streams::Thumbnail},
    error::YtuwuError,
    itags::{AnyItag, Itag},
};

// TODO: put thumbnail resolutions as comments
// TODO: Use this as itag maybe
/// This enum defines the different resolutions for the thumbnails
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ThumbRes {
    Highest,
    Low,
    Medium,
    High,
    VeryHigh,
}

/// Order containing every ThumbRes variant
/// Used for next best
const THUMBNAIL_ORDER: [ThumbRes; 5] = [ThumbRes::Highest, ThumbRes::VeryHigh, ThumbRes::High, ThumbRes::Medium, ThumbRes::Low];

impl Itag for ThumbRes {
    type Stream = Thumbnail;

    fn to_int(&self) -> u16 {
        panic!("thumbnail resolution can not be transformed into an itag int");
    }

    fn to_any(self) -> super::AnyItag {
        AnyItag::Thumbnail(self)
    }

    fn next_best(self) -> crate::Result<Self>
    where
        Self: Sized,
    {
        for (i, itag) in THUMBNAIL_ORDER.iter().enumerate() {
            if *itag == self {
                let next_itag = THUMBNAIL_ORDER
                    .get(i + 1)
                    .ok_or(YtuwuError::NoLowerItagFound)?;
                return Ok(*next_itag);
            }
        }
        panic!("Itag doesnt exit")
    }

    fn is_highest(&self) -> bool {
        *self == ThumbRes::Highest
    }

    fn new_stream(self) -> Self::Stream {
        Thumbnail::new()
    }

    fn get_mime_type(&self) -> crate::downloader::mime_types::MimeType {
        MimeType::Png
    }

    fn highest() -> Self
    where
        Self: Sized,
    {
        Self::VeryHigh
    }
}

impl ThumbRes {
    /// matches the given width and returns the correct thumbnail resolution as a variant of self
    /// Returns None if there was no matching resolution with that width.
    pub fn from_width(width: u16) -> Option<Self> {
        match width {
            120 => Some(Self::Low),
            320 => Some(Self::Medium),
            480 => Some(Self::High),
            640 => Some(Self::VeryHigh),
            _ => None,
        }
    }
}

impl Display for ThumbRes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw = match self {
            ThumbRes::Highest => "Highest",
            ThumbRes::Low => "Low",
            ThumbRes::Medium => "Medium",
            ThumbRes::High => "High",
            ThumbRes::VeryHigh => "VeryHigh",
        };
        write!(f, "Thumbnail resolution: {}", raw)
    }
}
