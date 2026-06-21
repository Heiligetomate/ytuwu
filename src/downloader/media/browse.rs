use std::sync::Arc;

use uuid::Uuid;

use crate::{
    Result,
    downloader::{Downloader, media::Media},
    request::api_captcha_bypass,
    types::VideoId,
};

/// This struct is used for browsing media with a video id
/// Holds the video id and an uuid for identification
#[derive(Debug, PartialEq, Eq)]
pub struct MediaBrowse {
    pub video_id: VideoId,
    pub id: Uuid,
}

impl MediaBrowse {
    /// Creates a new media browse
    pub fn new(video_id: VideoId, id: Uuid) -> Self {
        Self { video_id, id }
    }

    /// Uses the api_captcha_bypass function to bypass the captcha and get a response that can then
    /// be transformed into a Media object.
    pub async fn browse(self, downloader: Arc<Downloader>) -> Result<Media> {
        let response = api_captcha_bypass(&self.video_id, 5, &downloader.visitor_data, &downloader.client).await?;
        response.extract(downloader, self.id)
    }
}
