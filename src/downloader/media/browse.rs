use std::sync::Arc;

use uuid::Uuid;

use crate::{
    Result,
    downloader::{Downloader, media::Media},
    request::api_captcha_bypass,
    types::VideoId,
};

#[derive(Debug, PartialEq, Eq)]
pub struct MediaBrowse {
    pub video_id: VideoId,
    pub id: Uuid,
}

impl MediaBrowse {
    pub fn new(video_id: VideoId, id: Uuid) -> Self {
        Self { video_id, id }
    }

    pub async fn browse(self, downloader: Arc<Downloader>) -> Result<Media> {
        let response = api_captcha_bypass(&self.video_id, 5, &downloader.visitor_data, &downloader.client).await?;
        response.extract(downloader, self.id)
    }
}
