use std::sync::Arc;

use crate::{Downloader, Result, downloader::media::core::Media, request::core::api_captcha_bypass, types::VideoId};

#[derive(Debug, PartialEq, Eq)]
pub struct MediaBrowse {
    pub video_id: VideoId,
}

impl MediaBrowse {
    pub fn new(id: VideoId) -> Self {
        Self { video_id: id }
    }

    pub async fn browse(self, downloader: Arc<Downloader>) -> Result<Media> {
        let response = api_captcha_bypass(&self.video_id, 5, &downloader.visitor_data, &downloader.client).await?;
        response.extract(downloader)
    }
}
