use crate::{
    Id, Result,
    downloader::media::core::Media,
    id_types::{ShortId, VideoId},
    request::core::captcha_bypass,
};

#[derive(Debug)]
pub struct MediaBrowse {
    video_id: VideoId,
}

impl MediaBrowse {
    pub fn new(id: VideoId) -> Self {
        Self { video_id: id }
    }

    pub fn from_short(id: ShortId) -> Result<Self> {
        let video_id = VideoId::new(id.get_id())?;
        Ok(Self { video_id })
    }

    pub async fn browse(self) -> Result<Media> {
        let response = captcha_bypass(&self.video_id, 2).await?;
        response.extract()
    }
}
