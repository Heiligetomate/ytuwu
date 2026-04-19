use serde::{Deserialize, Serialize};

use crate::player_model::itag::Itag;

pub trait Stream {
    fn get_url(&self) -> &str;
    fn get_mime_type(&self) -> &str; 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StreamingData {
    expires_in_seconds: String,
    formats: Vec<MuxedStream>,
    adaptive_formats: Vec<AdaptiveStream>,
    server_abr_streaming_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MuxedStream {
    itag: u16,
    url: String,
    mime_type: String,
    bitrate: u32,
    average_bitrate: Option<u32>,
    width: Option<u16>,
    height: Option<u16>,
    fps: Option<u16>,
    quality: String,
    quality_label: Option<String>,
    audio_quality: String,
    audio_sample_rate: String,
    audio_channels: u32,
    content_length: Option<String>,
    approx_duration_ms: String,
    last_modified: String,
    projection_type: Option<String>,
    quality_ordinal: Option<String>,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdaptiveStream {
    itag: u16,
    url: String,
    pub mime_type: String,
    bitrate: u32,
    average_bitrate: Option<u32>,
    content_length: Option<String>,
    approx_duration_ms: String,
    last_modified: String,
    quality: String,
    high_replication: Option<bool>,
    projection_type: Option<String>,
    quality_ordinal: Option<String>,
    xtags: Option<String>,
    init_range: Option<Range>,
    index_range: Option<Range>,

    // video only
    width: Option<u32>,
    height: Option<u32>,
    fps: Option<u16>,
    quality_label: Option<String>,
    color_info: Option<ColorInfo>,

    // audio only
    audio_quality: Option<String>,
    audio_sample_rate: Option<String>,
    audio_channels: Option<u32>,
    loudness_db: Option<f64>,
    audio_track: Option<AudioTrack>,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioTrack {
    display_name: String,
    id: String,
    audio_is_default: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ColorInfo {
    primaries: String,
    transfer_characteristics: String,
    matrix_coefficients: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Range {
    start: String,
    end: String,
}


impl Stream for MuxedStream {
    fn get_url(&self) -> &str {
        self.url.as_ref()
    }
    fn get_mime_type(&self) -> &str {
        self.mime_type.as_ref()
    }
}

impl Stream for AdaptiveStream {
    fn get_url(&self) -> &str {
        self.url.as_ref()
    }
    fn get_mime_type(&self) -> &str {
        self.mime_type.as_ref()
    }
}

impl StreamingData {
    pub fn get_url_by_itag(&self, itag: &impl Itag) -> Option<&str> {
        for format in self.adaptive_formats.iter() {
            if format.itag == itag.to_int() {
                return Some(format.get_url());
            }
        }
        for adaptive_format in self.formats.iter() {
            if adaptive_format.itag == itag.to_int() {
                return Some(adaptive_format.get_url());
            }
        }
        None
    }

    pub fn get_stream_by_itag(&self, itag: &impl Itag) -> Option<&dyn Stream> {
        for muxed_stream in &self.formats {
            if muxed_stream.itag == itag.to_int() { 
                return Some(muxed_stream as &dyn Stream); 
            }
        }
        for adaptive_stream in &self.adaptive_formats {
            if adaptive_stream.itag == itag.to_int() { 
                return Some(adaptive_stream as &dyn Stream); 
            }
        }
        None
    }
}
