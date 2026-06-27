use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    Result,
    downloader::{itags::core::Itag, mime_types::MimeType, streams::VideoStream},
    error::YtuwuError,
};

/// VideoItag contains all pure Video streams.
/// It also contains a Highest variant which should be used if the stream quality should be
/// downgraded to avoid non existent streams
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum VideoItag {
    Highest,

    Webm2160p60HDR, // 337
    Webm2160p60,    // 315
    Webm1440p60HDR, // 336
    Webm1440p60,    // 308
    MP41080p,       // 137
    WebM1080p,      // 248
    MP41080p50,     // 299
    Webm1080p50,    // 303
    MP4720p,        // 136
    WebM720p,       // 247
    MP4720p50,      // 298
    Webm720p50,     // 302
    MP4480p,        // 135
    Webm480p,       // 244
    MP4360p,        // 134
    WebM360p,       // 243
    MP4240p,        // 133
    WebM240p,       // 242
    MP4144p,        // 160
    Webm144p,       // 278
}

/// Order containing every VideoItag variant
/// Used for next best
const LONG_VIDEO_ORDER: [VideoItag; 21] = [
    VideoItag::Highest,
    VideoItag::Webm2160p60HDR,
    VideoItag::Webm2160p60,
    VideoItag::Webm1440p60HDR,
    VideoItag::Webm1440p60,
    VideoItag::WebM1080p,
    VideoItag::MP41080p,
    VideoItag::Webm1080p50,
    VideoItag::MP41080p50,
    VideoItag::WebM720p,
    VideoItag::MP4720p,
    VideoItag::Webm720p50,
    VideoItag::MP4720p50,
    VideoItag::Webm480p,
    VideoItag::MP4480p,
    VideoItag::WebM360p,
    VideoItag::MP4360p,
    VideoItag::WebM240p,
    VideoItag::MP4240p,
    VideoItag::Webm144p,
    VideoItag::MP4144p,
];

impl Itag for VideoItag {
    type Stream = VideoStream;

    fn is_highest(&self) -> bool {
        *self == Self::Highest
    }

    fn highest() -> Self {
        Self::Webm2160p60HDR
    }

    fn next_best(self) -> Result<Self>
    where
        Self: Sized,
    {
        for (i, itag) in LONG_VIDEO_ORDER.iter().enumerate() {
            if *itag == self {
                let next_itag = LONG_VIDEO_ORDER
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
            Self::Webm2160p60HDR => 337,
            Self::Webm2160p60 => 315,
            Self::Webm1440p60HDR => 336,
            Self::Webm1440p60 => 308,
            Self::WebM1080p => 248,
            Self::MP41080p => 137,
            Self::MP41080p50 => 299,
            Self::Webm1080p50 => 303,
            Self::WebM720p => 247,
            Self::MP4720p => 136,
            Self::MP4720p50 => 298,
            Self::Webm720p50 => 302,
            Self::Webm480p => 244,
            Self::MP4480p => 135,
            Self::WebM360p => 243,
            Self::MP4360p => 134,
            Self::WebM240p => 242,
            Self::MP4240p => 133,
            Self::Webm144p => 278,
            Self::MP4144p => 160,
        }
    }

    fn get_mime_type(&self) -> MimeType {
        match &self {
            Self::Highest => Self::highest().get_mime_type(),
            Self::Webm2160p60HDR => MimeType::Webm,
            Self::Webm2160p60 => MimeType::Webm,
            Self::Webm1440p60HDR => MimeType::Webm,
            Self::Webm1440p60 => MimeType::Webm,
            Self::WebM1080p => MimeType::Webm,
            Self::MP41080p => MimeType::MP4,
            Self::MP41080p50 => MimeType::MP4,
            Self::Webm1080p50 => MimeType::Webm,
            Self::WebM720p => MimeType::Webm,
            Self::MP4720p => MimeType::MP4,
            Self::MP4720p50 => MimeType::MP4,
            Self::Webm720p50 => MimeType::Webm,
            Self::Webm480p => MimeType::Webm,
            Self::MP4480p => MimeType::MP4,
            Self::WebM360p => MimeType::Webm,
            Self::MP4360p => MimeType::MP4,
            Self::WebM240p => MimeType::Webm,
            Self::MP4240p => MimeType::MP4,
            Self::Webm144p => MimeType::Webm,
            Self::MP4144p => MimeType::MP4,
        }
    }

    fn new_stream(self) -> Self::Stream {
        VideoStream::new(self)
    }

    fn to_any(self) -> super::AnyItag {
        super::AnyItag::Video(self)
    }
}

impl Display for VideoItag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw = match self {
            VideoItag::Highest => "Highest",
            VideoItag::Webm2160p60HDR => "Webm2160p60HDR",
            VideoItag::Webm2160p60 => "Webm2160p60",
            VideoItag::Webm1440p60HDR => "Webm1440p60HDR",
            VideoItag::Webm1440p60 => "Webm2160p60",
            VideoItag::MP41080p => "MP41080p",
            VideoItag::WebM1080p => "WebM1080p",
            VideoItag::MP41080p50 => "Webm1080p50",
            VideoItag::Webm1080p50 => "Webm1080p50",
            VideoItag::MP4720p => "MP4720p",
            VideoItag::WebM720p => "WebM720p",
            VideoItag::MP4720p50 => "MP4720p50",
            VideoItag::Webm720p50 => "Webm720p50",
            VideoItag::MP4480p => "MP4480p",
            VideoItag::Webm480p => "Webm480p",
            VideoItag::MP4360p => "MP$WebM360p",
            VideoItag::WebM360p => "WebM360p",
            VideoItag::MP4240p => "MP4240p",
            VideoItag::WebM240p => "WebM240p",
            VideoItag::MP4144p => "MP4144p",
            VideoItag::Webm144p => "Webm144p",
        };
        write!(f, "Video: {}", raw)
    }
}
