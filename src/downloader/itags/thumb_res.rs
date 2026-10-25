use std::fmt::Display;

use crate::{
    downloader::{mime_types::MimeType, streams::Thumbnail},
    error::YtuwuError,
    itags::{AnyItag, Itag},
};

// TODO: better naming
/// This enum defines the different resolutions for the thumbnails.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ThumbRes {
    Highest,
    /// 120x90
    Low,
    /// 320x180
    Medium,
    /// 480x360
    High,
    /// 640x480
    VeryHigh,
    ///1280x720
    UltraHigh,
}

/// Order containing every ThumbRes variant.
/// Used for next best.
const THUMBNAIL_ORDER: [ThumbRes; 6] = [ThumbRes::Highest, ThumbRes::UltraHigh, ThumbRes::VeryHigh, ThumbRes::High, ThumbRes::Medium, ThumbRes::Low];

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
        Self::UltraHigh
    }
}

impl ThumbRes {
    /// Returns the width this resolution aims for.
    /// Returns None for `Highest`, which has no fixed width and means "the widest available".
    pub(crate) fn target_width(&self) -> Option<u16> {
        match self {
            Self::Highest => None,
            Self::Low => Some(120),
            Self::Medium => Some(320),
            Self::High => Some(480),
            Self::VeryHigh => Some(640),
            Self::UltraHigh => Some(1280),
        }
    }

    /// Matches the given width and returns the correct thumbnail resolution as a variant of self.
    /// Returns None if there was no matching resolution with that width.
    pub fn from_width(width: u16) -> Option<Self> {
        match width {
            120 => Some(Self::Low),
            320 => Some(Self::Medium),
            480 => Some(Self::High),
            640 => Some(Self::VeryHigh),
            1280 => Some(Self::UltraHigh),
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
            ThumbRes::VeryHigh => "Very High",
            ThumbRes::UltraHigh => "Ultra High",
        };
        write!(f, "Thumbnail resolution: {}", raw)
    }
}
