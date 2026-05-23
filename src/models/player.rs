use serde::Deserialize;

use crate::{
    downloader::media::extracted_streams::ExtractedPlayerResponse,
    error::{Result, YtuwuError},
    models::response::{Response, Status},
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
struct ResponseContext {
    pub visitor_data: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct StreamingData {
    formats: Vec<Stream>,
    adaptive_formats: Vec<Stream>,
}

#[derive(Deserialize, Debug)]
pub struct Stream {
    pub itag: u16,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct VideoDetails {
    pub title: String,
    pub author: String,
    pub thumbnail: Thumbnails,
}

#[derive(Deserialize, Debug)]
pub struct Thumbnails {
    pub thumbnails: Vec<ThumbnailStream>,
}

#[derive(Deserialize, Debug)]
pub struct ThumbnailStream {
    pub url: String,
    pub width: u16,
}

#[derive(Deserialize, Debug)]
struct PlayabilityStatus {
    pub status: PlayabilityStatusValue,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlayabilityStatusValue {
    Ok,
    LoginRequired,
    Error,
    Unplayable,
    LiveStreamOffline,
    ContentCheckRequired,
}

impl PlayerResponse {
    pub fn extract(self) -> Result<ExtractedPlayerResponse> {
        let streaming_data = self
            .streaming_data
            .ok_or(YtuwuError::PlayerDataNotFound("Streaming data"))?;

        let mut media_streams = streaming_data.formats;
        media_streams.extend(streaming_data.adaptive_formats);

        let video_details = self
            .video_details
            .ok_or(YtuwuError::PlayerDataNotFound("video details"))?;

        Ok(ExtractedPlayerResponse::new(media_streams, video_details))
    }
}

impl Response for PlayerResponse {
    fn get_visitor_data(&self) -> Option<&str> {
        if let Some(response_context) = &self.response_context {
            return response_context
                .visitor_data
                .as_ref()
                .map(|vd| vd.as_str());
        }
        None
    }

    fn get_status(&self) -> Status {
        if let Some(playability_status) = &self.playability_status {
            return match playability_status.status {
                PlayabilityStatusValue::Ok => Status::Success,
                PlayabilityStatusValue::LoginRequired => Status::Login,
                _ => Status::Error,
            };
        }
        Status::Error
    }
}
