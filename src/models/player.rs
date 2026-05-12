use serde::{Deserialize, Serialize};

use crate::{
    error::{Result, YtuwuError},
    models::itag::Itag,
    shared_traits::{self, Response},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
    response_context: Option<ResponseContext>,
    playability_status: Option<PlayabilityStatus>,
    streaming_data: Option<StreamingData>,
    video_details: Option<VideoDetails>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseContext {
    pub visitor_data: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StreamingData {
    formats: Vec<Stream>,
    adaptive_formats: Vec<Stream>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    itag: u16,
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct VideoDetails {
    pub title: String,
    pub author: String,
    pub thumbnail: Thumbnails,
}

#[derive(Deserialize, Debug)]
pub struct Thumbnails {
    thumbnails: Vec<Thumbnail>,
}

#[derive(Deserialize, Debug)]
pub struct Thumbnail {
    url: String,
    width: u16,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayabilityStatus {
    pub status: PlayabilityStatusValue,
    reason: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ThumbnailResolution {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlayabilityStatusValue {
    Ok,
    LoginRequired,
    Error,
    Unplayable,
    LiveStreamOffline,
    ContentCheckRequired,
}

impl ThumbnailResolution {
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

impl Thumbnails {
    pub fn url_by_resolution(&self, resolution: &ThumbnailResolution) -> Option<&str> {
        for thumbnail in self.thumbnails.iter() {
            if let Some(thumbnail_resolution) = ThumbnailResolution::from_width(thumbnail.width) {
                if thumbnail_resolution == *resolution {
                    return Some(&thumbnail.url);
                }
            } else {
                return None;
            }
        }
        None
    }
}

impl PlayerResponse {
    pub fn get_best_stream<I: Itag + Copy>(&self, itag: &I) -> Result<&str> {
        let streams = self.get_streaming_data()?;
        let mut current_itag = *itag;
        let mut url: Option<&str> = streams.get_url_by_itag(&current_itag);
        while url.is_none() {
            current_itag = current_itag.next_best()?;
            url = streams.get_url_by_itag(&current_itag);
        }
        Ok(url.ok_or(YtuwuError::NoMatchingItag)?)
    }

    fn get_streaming_data(&self) -> Result<&StreamingData> {
        Ok(self
            .streaming_data
            .as_ref()
            .ok_or(YtuwuError::PlayerDataNotFound("streaming data"))?)
    }

    fn get_video_deatails(&self) -> Result<&VideoDetails> {
        Ok(self
            .video_details
            .as_ref()
            .ok_or(YtuwuError::PlayerDataNotFound("video details"))?)
    }

    pub fn get_thumbnail_url_by_res(&self, resolution: &ThumbnailResolution) -> Result<&str> {
        let url = self
            .get_video_deatails()?
            .thumbnail
            .url_by_resolution(resolution)
            .ok_or(YtuwuError::PlayerDataNotFound("thumbnails"))?;
        Ok(url)
    }

    pub fn get_title(&self) -> Result<&str> {
        Ok(&self.get_video_deatails()?.title)
    }

    pub fn get_author(&self) -> Result<&str> {
        if let Some(video_details) = &self.video_details {
            return Ok(&video_details.author);
        }
        Err(YtuwuError::PlayerDataNotFound("author"))
    }
}

impl StreamingData {
    pub fn get_url_by_itag(&self, itag: &impl Itag) -> Option<&str> {
        for format in self.adaptive_formats.iter() {
            if format.itag == itag.to_int() {
                return Some(&format.url);
            }
        }
        for adaptive_format in self.formats.iter() {
            if adaptive_format.itag == itag.to_int() {
                return Some(&adaptive_format.url);
            }
        }
        None
    }
}

impl Response for PlayerResponse {
    fn get_visitor_data(&self) -> Option<String> {
        if let Some(response_context) = &self.response_context {
            return response_context.visitor_data.clone();
        }
        None
    }

    fn get_status(&self) -> shared_traits::Status {
        if let Some(playability_status) = &self.playability_status {
            return match playability_status.status {
                PlayabilityStatusValue::Ok => shared_traits::Status::Success,
                PlayabilityStatusValue::LoginRequired => shared_traits::Status::Login,
                _ => shared_traits::Status::Error,
            };
        }
        shared_traits::Status::Error
    }
}
