use std::sync::Arc;

use serde::Deserialize;
use uuid::Uuid;

use crate::{
    downloader::{
        Downloader,
        media::{ExtractedStreams, ExtractedThumbnails, Media},
        metadata::MediaMetadata,
    },
    error::{Result, YtuwuError},
    models::response::{Response, Status},
};

/// This is response when using the player client
/// It is used for getting all the downlaodable streams for a media
/// It has the basic structure to extract all streams, visitor data and playability status  
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
    response_context: Option<ResponseContext>,
    playability_status: Option<PlayabilityStatus>,
    streaming_data: Option<StreamingData>,
    video_details: Option<VideoDetails>,
}

/// contains the important visitor data
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ResponseContext {
    visitor_data: Option<String>,
}

/// This struct contains two different types of streams that can be merged into on large collection
/// of streams later
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct StreamingData {
    formats: Vec<Stream>,
    adaptive_formats: Vec<Stream>,
}

/// A stream always contains an itag for identifying what kind of stream it is and an url that
/// contains the downloadable media stream  
#[derive(Deserialize, Debug)]
pub struct Stream {
    pub itag: u16,
    pub url: String,
}

/// This struct contains basic information about the media: title and author
/// It also contains a thumbnail field for the different thumbnail resolutions
#[derive(Deserialize, Debug)]
pub struct VideoDetails {
    pub title: String,
    pub author: String,
    pub thumbnail: Thumbnails,
}

/// This struct contains multiple thumbnail streams with different resolutions
#[derive(Deserialize, Debug)]
pub struct Thumbnails {
    pub thumbnails: Vec<ThumbnailStream>,
}

/// thumbnail streams are stored in the thumbnail struct
/// They contain a width to identify the resolution and an url for downloading the thumbnail
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

// TODO: This is useless right now
/// This struct contains more statuses than the normal response status.
/// It tries to deserialize the raw response into on of these variants
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
    /// This function extracts all streams (both formats, get merged), some basic video details that
    /// get stored in the metadata later and all thumbnail streams. It merges all of those data into
    /// a media object
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

    /// This function returns the error reason
    /// Only usecase is the "age verification" status right now
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

    /// This function tries to get the visitor data and returns an option
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
