use std::sync::Arc;

use crate::{Downloader, Result, downloader::channel::core::ChannelContentBrowse, id_resolver::types::ChannelId, request::core::api_request};

pub struct ChannelBrowse {
    id: ChannelId,
    downloader: Arc<Downloader>,
}

impl ChannelBrowse {
    pub async fn new(channel_id: ChannelId, downloader: Arc<Downloader>) -> Result<Self> {
        let channel_id = channel_id
            .make_valid(&downloader.client)
            .await?;
        Ok(Self { id: channel_id, downloader })
    }

    pub async fn browse(self) -> Result<ChannelContentBrowse> {
        let resp = api_request(&self.id, &self.downloader.client).await?;
        resp.extract_all_releases(self.downloader)
    }
}
