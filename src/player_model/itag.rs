use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Itag {
    // muxed
    MuxedMP4, // 18

    // video only
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
    
    // shorts vertical video only (608x1080)
    ShortLow, // 779
    Short,      // 780

    // audio only
    AacLow,     // 139
    AacMedium,  // 140
    OpusLow,    // 249
    OpusMedium, // 251
}

impl Itag {
    pub fn from_int(itag: u16) -> Result<Self, String> {
        match itag {
            18 => Ok(Self::MuxedMP4),
            137 => Ok(Self::MP41080p),
            248 => Ok(Self::WebM1080p),
            136 => Ok(Self::MP4720p),
            247 => Ok(Self::WebM720p),
            135 => Ok(Self::MP4480p),
            244 => Ok(Self::Webm480p),
            134 => Ok(Self::MP4360p),
            243 => Ok(Self::WebM360p),
            133 => Ok(Self::MP4240p),
            242 => Ok(Self::WebM240p),
            160 => Ok(Self::MP4144p),
            278 => Ok(Self::Webm144p),
            779 => Ok(Self::ShortLow),
            780 => Ok(Self::Short),
            139 => Ok(Self::AacLow),
            140 => Ok(Self::AacMedium),
            249 => Ok(Self::OpusLow),
            251 => Ok(Self::OpusMedium),
            _ => Err(String::from("Itag does not exist")),
        }
    }
    pub fn to_string(&self) -> String {
        let itag_id = self.to_int();
        format!("Itag {}", itag_id)
    }
    pub fn to_int(&self) -> u16 {
        match &self {
            Self::MuxedMP4 => 18,
            Self::MP41080p => 137,
            Self::WebM1080p => 248,
            Self::MP4720p => 136,
            Self::WebM720p => 247,
            Self::MP4480p => 135,
            Self::Webm480p => 244,
            Self::MP4360p => 134,
            Self::WebM360p => 243,
            Self::MP4240p => 133,
            Self::WebM240p => 242,
            Self::MP4144p => 160,
            Self::Webm144p => 278,
            Self::ShortLow => 779,
            Self::Short => 780,
            Self::AacLow => 139,
            Self::AacMedium => 140,
            Self::OpusLow => 249,
            Self::OpusMedium => 251,
        }
    }
}
