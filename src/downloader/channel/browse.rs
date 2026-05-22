use crate::{Result, downloader::channel::content_browse::ChannelContentBrowse, id_resolver::id_types::channel_id::ChannelId, request::core::captcha_bypass};

pub struct ChannelBrowse {
    id: ChannelId,
}

impl ChannelBrowse {
    pub fn new(channel_id: ChannelId) -> Self {
        Self { id: channel_id }
    }

    pub async fn browse(self) -> Result<ChannelContentBrowse> {
        let resp = captcha_bypass(&self.id, 1).await?;
        resp.extract_all_releases()
    }
}
