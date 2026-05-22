use crate::{
    Result,
    downloader::media::core::Media,
    id_resolver::{id::Id, id_types::ShortId, id_types::VideoId},
    name_trimmer::trim,
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

    pub fn from_short(id: ShortId) -> Self {
        let video_id = VideoId::new(id.get_id());
        Self { video_id }
    }

    pub async fn browse(self) -> Result<Media> {
        let response = captcha_bypass(&self.video_id, 2).await?;
        let title = response.get_title()?.to_owned();
        let trimmed_title = trim(title, "-");
        Ok(Media::new(response, &trimmed_title))
    }
}
