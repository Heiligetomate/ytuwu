use crate::{Result, downloader::channel::core::ChannelContentBrowse, id_resolver::types::ChannelId, request::core::api_request};

pub struct ChannelBrowse {
    id: ChannelId,
}

impl ChannelBrowse {
    pub fn new(channel_id: ChannelId) -> Self {
        Self { id: channel_id }
    }

    pub async fn browse(self) -> Result<ChannelContentBrowse> {
        let resp = api_request(&self.id).await?;
        resp.extract_all_releases()
    }
}
