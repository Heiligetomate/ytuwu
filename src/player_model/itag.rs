use bytes::Bytes;
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result, Error};

use crate::downloader::media_stream::{MediaStream, VideoStream};

pub trait Itag {
    fn highest() -> Self;
    fn next_best(self) -> Result<Self> where Self: Sized;

    fn to_int(&self) -> u16;
    fn get_mime_type(&self) -> &str;

    fn new_stream(&self) -> Box<&dyn MediaStream>;
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum MuxedItag {
    MuxedMP4
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum VideoItag {
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
    Low, // 779
    High,      // 780
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum AudioItag {
    AacLow,     // 139
    AacMedium,  // 140
    OpusLow,    // 249
    OpusMedium, // 251
}

const SHORT_ORDER: [ShortVideoItag; 2] = [
    ShortVideoItag::High, 
    ShortVideoItag::Low
]; 

const AUDIO_ORDER: [AudioItag; 4] = [
    AudioItag::OpusMedium, 
    AudioItag::AacMedium, 
    AudioItag::OpusLow, 
    AudioItag::AacLow
];

const VIDEO_ORDER: [VideoItag; 12] = [
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
    VideoItag::MP4144p, 
    VideoItag::MP4144p
];

fn no_high_itag_found() -> Error {
    anyhow!("No higher itag found.")
}

impl Itag for VideoItag {

    fn highest() -> Self {
        Self::WebM1080p
    }

    fn next_best(self) -> Result<Self> where Self: Sized {
        for (i, itag) in VIDEO_ORDER.iter().enumerate() {
            if *itag == self {
                let next_itag = VIDEO_ORDER.get(i + 1).ok_or(no_high_itag_found())?;
                return Ok(*next_itag);
            }
        }
        panic!("Itag doesnt exit")
    }

    fn to_int(&self) -> u16 {
        match &self {
            Self::WebM1080p => 248, 
            Self::MP41080p  => 137, 
            Self::WebM720p  => 247, 
            Self::MP4720p   => 136, 
            Self::Webm480p  => 244, 
            Self::MP4480p   => 135, 
            Self::WebM360p  => 243, 
            Self::MP4360p   => 134, 
            Self::WebM240p  => 242,
            Self::MP4240p   => 133, 
            Self::Webm144p  => 278, 
            Self::MP4144p   => 160,
        }
    }

    fn get_mime_type(&self) -> &str {
        match &self {
            Self::WebM1080p => "webm", 
            Self::MP41080p  => "mp4", 
            Self::WebM720p  => "webm", 
            Self::MP4720p   => "mp4", 
            Self::Webm480p  => "webm", 
            Self::MP4480p   => "mp4", 
            Self::WebM360p  => "webm", 
            Self::MP4360p   => "mp4", 
            Self::WebM240p  => "webm",
            Self::MP4240p   => "mp4", 
            Self::Webm144p  => "webm", 
            Self::MP4144p   => "mp4",
        }
    }

    fn new_stream<M: MediaStream>(&self) -> Box<&dyn MediaStream> {
        Box::new(VideoStream::new(*self) as &dyn MediaStream)
    }

impl Itag for AudioItag {
    
    fn highest() -> Self {
        Self::OpusMedium
    }
    
    fn next_best(self) -> Result<Self> where Self: Sized {
        for (i, itag) in AUDIO_ORDER.iter().enumerate() {
            if *itag == self {
                let next_itag = AUDIO_ORDER.get(i + 1).ok_or(no_high_itag_found())?;
                return Ok(*next_itag)
            }
        }
        panic!("Itag doesnt exit")
    }

    fn to_int(&self) -> u16 {
        match &self {
            Self::OpusMedium => 251,
            Self::OpusLow    => 249,
            Self::AacMedium  => 140, 
            Self::AacLow     => 139,
        }
    }

    fn get_mime_type(&self) -> &str {
        match &self {
            Self::OpusMedium => "webm",
            Self::OpusLow    => "webm",
            Self::AacMedium  => "m4a",
            Self::AacLow     => "m4a",
        }
    }
}

impl Itag for ShortVideoItag {
    
    fn highest() -> Self {
        Self::High
    }

    fn next_best(self) -> Result<Self> where Self: Sized {
        for (i, itag) in SHORT_ORDER.iter().enumerate() {
            if *itag == self {
                let next_itag = SHORT_ORDER.get(i + 1).ok_or(no_high_itag_found())?;
                return Ok(*next_itag)
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
}

impl Itag for MuxedItag {

    fn highest() -> Self {
        Self::MuxedMP4
    }

    fn to_int(&self) -> u16 {
        18
    }
    
    fn next_best(self) -> Result<Self> {
        Err(no_high_itag_found())
    }
    
    fn get_mime_type(&self) -> &str {
        "mp4" 
    }
}

