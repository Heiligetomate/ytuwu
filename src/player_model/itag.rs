use serde::{Deserialize, Serialize};

use crate::{
    downloader::media_stream::{AudioStream, LongVideoStream, MediaStream, MuxedStream, ShortVideoStream, VideoStream},
    error::{Result, YtuwuError},
};

pub trait Itag {
    type Stream: MediaStream;

    fn highest() -> Self;
    fn next_best(self) -> Result<Self>
    where
        Self: Sized;

    fn to_int(&self) -> u16;
    fn get_mime_type(&self) -> &str;

    fn new_stream(self) -> Self::Stream;
}

pub trait VideoItag: Itag
where
    <Self as Itag>::Stream: VideoStream,
{
}

impl VideoItag for LongVideoItag {}

impl VideoItag for ShortVideoItag {}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum MuxedItag {
    MuxedMP4,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum LongVideoItag {
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
pub enum ShortVideoItag {
    Low,  // 779
    High, // 780
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum AudioItag {
    AacLow,     // 139
    AacMedium,  // 140
    OpusLow,    // 249
    OpusMedium, // 251
}

const SHORT_ORDER: [ShortVideoItag; 2] = [ShortVideoItag::High, ShortVideoItag::Low];

const AUDIO_ORDER: [AudioItag; 4] = [AudioItag::OpusMedium, AudioItag::AacMedium, AudioItag::OpusLow, AudioItag::AacLow];

const VIDEO_ORDER: [LongVideoItag; 12] = [
    LongVideoItag::WebM1080p,
    LongVideoItag::MP41080p,
    LongVideoItag::WebM720p,
    LongVideoItag::MP4720p,
    LongVideoItag::Webm480p,
    LongVideoItag::MP4480p,
    LongVideoItag::WebM360p,
    LongVideoItag::MP4360p,
    LongVideoItag::WebM240p,
    LongVideoItag::MP4240p,
    LongVideoItag::MP4144p,
    LongVideoItag::MP4144p,
];

impl Itag for LongVideoItag {
    type Stream = LongVideoStream;

    fn highest() -> Self {
        Self::WebM1080p
    }

    fn next_best(self) -> Result<Self>
    where
        Self: Sized,
    {
        for (i, itag) in VIDEO_ORDER.iter().enumerate() {
            if *itag == self {
                let next_itag = VIDEO_ORDER
                    .get(i + 1)
                    .ok_or(YtuwuError::NoLowerItagFound)?;
                return Ok(*next_itag);
            }
        }
        panic!("Itag doesnt exit")
    }

    fn to_int(&self) -> u16 {
        match &self {
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

    fn get_mime_type(&self) -> &str {
        match &self {
            Self::WebM1080p => "webm",
            Self::MP41080p => "mp4",
            Self::WebM720p => "webm",
            Self::MP4720p => "mp4",
            Self::Webm480p => "webm",
            Self::MP4480p => "mp4",
            Self::WebM360p => "webm",
            Self::MP4360p => "mp4",
            Self::WebM240p => "webm",
            Self::MP4240p => "mp4",
            Self::Webm144p => "webm",
            Self::MP4144p => "mp4",
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
            Self::OpusMedium => 251,
            Self::OpusLow => 249,
            Self::AacMedium => 140,
            Self::AacLow => 139,
        }
    }

    fn get_mime_type(&self) -> &str {
        match &self {
            Self::OpusMedium => "webm",
            Self::OpusLow => "webm",
            Self::AacMedium => "m4a",
            Self::AacLow => "m4a",
        }
    }

    fn new_stream(self) -> Self::Stream {
        AudioStream::new(self)
    }
}

impl Itag for ShortVideoItag {
    type Stream = ShortVideoStream;

    fn highest() -> Self {
        Self::High
    }

    fn next_best(self) -> Result<Self>
    where
        Self: Sized,
    {
        for (i, itag) in SHORT_ORDER.iter().enumerate() {
            if *itag == self {
                let next_itag = SHORT_ORDER
                    .get(i + 1)
                    .ok_or(YtuwuError::NoLowerItagFound)?;
                return Ok(*next_itag);
            }
        }
        panic!("Itag doesnt exit")
    }

    fn to_int(&self) -> u16 {
        match &self {
            Self::Low => 779,
            Self::High => 780,
        }
    }

    fn get_mime_type(&self) -> &str {
        "mp4"
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

    fn to_int(&self) -> u16 {
        18
    }

    fn next_best(self) -> Result<Self> {
        Err(YtuwuError::NoLowerItagFound)
    }

    fn get_mime_type(&self) -> &str {
        "mp4"
    }

    fn new_stream(self) -> Self::Stream {
        MuxedStream::new(self)
    }
}
