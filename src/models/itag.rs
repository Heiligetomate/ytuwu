use serde::{Deserialize, Serialize};

use crate::{
    downloader::{
        media_stream::{AudioStream, LongVideoStream, MediaStream, MuxedStream, ShortVideoStream},
        mime_types::MimeType,
    },
    error::{Result, YtuwuError},
};

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

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum MuxedItag {
    Highest,
    MuxedMP4,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum VideoItag {
    Highest,
    MP41080p,  // 137
    WebM1080p, // 248
    MP4720p,   // 136
    WebM720p,  // 247
    MP4480p,   // 135
    Webm480p,  // 244
    MP4360p,   // 134
    WebM360p,  // 243
    MP4240p,   // 133
    WebM240p,  // 242
    MP4144p,   // 160
    Webm144p,  // 278
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum ShortItag {
    Highest,
    Low,  // 779
    High, // 780
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum AudioItag {
    Highest,
    AacLow,     // 139
    AacMedium,  // 140
    OpusLow,    // 249
    OpusMedium, // 251
}

const SHORT_LONG_VIDEO_ORDER: [ShortItag; 2] = [ShortItag::High, ShortItag::Low];

const AUDIO_ORDER: [AudioItag; 4] = [AudioItag::OpusMedium, AudioItag::AacMedium, AudioItag::OpusLow, AudioItag::AacLow];

const LONG_VIDEO_ORDER: [VideoItag; 12] = [
    VideoItag::WebM1080p,
    VideoItag::MP41080p,
    VideoItag::WebM720p,
    VideoItag::MP4720p,
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
    type Stream = LongVideoStream;

    fn is_highest(&self) -> bool {
        *self == Self::Highest
    }

    fn highest() -> Self {
        Self::WebM1080p
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
            Self::WebM1080p => 248,
            Self::MP41080p => 137,
            Self::WebM720p => 247,
            Self::MP4720p => 136,
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
            Self::WebM1080p => MimeType::Webm,
            Self::MP41080p => MimeType::MP4,
            Self::WebM720p => MimeType::Webm,
            Self::MP4720p => MimeType::MP4,
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
        LongVideoStream::new(self)
    }
}

impl Itag for AudioItag {
    type Stream = AudioStream;

    fn highest() -> Self {
        Self::OpusMedium
    }

    fn is_highest(&self) -> bool {
        *self == Self::Highest
    }

    fn next_best(self) -> Result<Self>
    where
        Self: Sized,
    {
        for (i, itag) in AUDIO_ORDER.iter().enumerate() {
            if *itag == self {
                let next_itag = AUDIO_ORDER
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
            Self::OpusMedium => 251,
            Self::OpusLow => 249,
            Self::AacMedium => 140,
            Self::AacLow => 139,
        }
    }

    fn get_mime_type(&self) -> MimeType {
        match &self {
            Self::Highest => Self::highest().get_mime_type(),
            Self::OpusMedium => MimeType::Webm,
            Self::OpusLow => MimeType::Webm,
            Self::AacMedium => MimeType::M4A,
            Self::AacLow => MimeType::M4A,
        }
    }

    fn new_stream(self) -> Self::Stream {
        AudioStream::new(self)
    }
}

impl Itag for ShortItag {
    type Stream = ShortVideoStream;

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
        ShortVideoStream::new(self)
    }
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
}

#[derive(Debug, Clone, Copy)]
pub enum AnyItag {
    Audio(AudioItag),
    LongVideo(VideoItag),
    ShortVideo(ShortItag),
    Muxed(MuxedItag),
}
