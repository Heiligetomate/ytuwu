use crate::{Result, downloader::media::core::Media, id_types::VideoId, request::core::api_captcha_bypass};

#[derive(Debug)]
pub struct MediaBrowse {
    video_id: VideoId,
}

impl MediaBrowse {
    pub fn new(id: VideoId) -> Self {
        Self { video_id: id }
    }

    pub async fn browse(self) -> Result<Media> {
        let response = api_captcha_bypass(&self.video_id, 2).await?;
        response.extract()
    }
}
