use serde::{Deserialize, Serialize};

use crate::{
    Result,
    downloader::{itags::core::Itag, mime_types::MimeType, streams::AudioStream},
    error::YtuwuError,
};

/// AudioItag contains all pure Audio streams.
/// It also contains a Highest variant which should be used if the stream quality should be
/// downgraded to avoid non existent streams
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum AudioItag {
    Highest,
    AacLow,     // 139
    AacMedium,  // 140
    OpusLow,    // 249
    OpusMedium, // 251
}

/// Order containing every AudioItag variant
/// Used for next best
const AUDIO_ORDER: [AudioItag; 5] = [AudioItag::Highest, AudioItag::OpusMedium, AudioItag::AacMedium, AudioItag::OpusLow, AudioItag::AacLow];

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

    fn to_any(self) -> super::AnyItag {
        super::AnyItag::Audio(self)
    }
}
