use std::sync::Arc;

use serde::Deserialize;
use uuid::Uuid;

use crate::{
    Downloader,
    downloader::media::{
        core::Media,
        extracted_streams::{ExtractedStreams, ExtractedThumbnails},
    },
    error::{Result, YtuwuError},
    metadata::MediaMetadata,
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
    visitor_data: Option<String>,
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
    status: PlayabilityStatusValue,
    reason: Option<String>,
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
    pub fn extract(self, downloader: Arc<Downloader>, id: Uuid) -> Result<Media> {
        let streaming_data = self
            .streaming_data
            .ok_or(YtuwuError::PlayerDataNotFound("Streaming data"))?;

        let mut extracted_streams = streaming_data.formats;
        extracted_streams.extend(streaming_data.adaptive_formats);
        let media_streams = ExtractedStreams::new(extracted_streams);

        let video_details = self
            .video_details
            .ok_or(YtuwuError::PlayerDataNotFound("video details"))?;

        let thumbnail_streams = ExtractedThumbnails::new(video_details.thumbnail.thumbnails);
        let metadata = MediaMetadata::new(&video_details.title, &video_details.author);

        Ok(Media::new(media_streams, thumbnail_streams, metadata, downloader, id))
    }

    pub fn get_playability_reason(&self) -> Result<&str> {
        let mesage = self
            .playability_status
            .as_ref()
            .ok_or(YtuwuError::PlayerDataNotFound("playability_status"))?
            .reason
            .as_ref()
            .ok_or(YtuwuError::PlayerDataNotFound("status message"))?;
        Ok(mesage)
    }

    pub fn get_visitor_data(&self) -> Option<&str> {
        if let Some(response_context) = &self.response_context {
            return response_context
                .visitor_data
                .as_ref()
                .map(|vd| vd.as_str());
        }
        None
    }
}

impl Response for PlayerResponse {
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
