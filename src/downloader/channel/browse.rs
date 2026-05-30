use std::sync::Arc;

use crate::{Downloader, Result, downloader::channel::core::ChannelContentBrowse, id_resolver::types::ChannelId, request::core::api_request};

pub struct ChannelBrowse {
    id: ChannelId,
    downloader: Arc<Downloader>,
}

impl ChannelBrowse {
    pub fn new(channel_id: ChannelId, downloader: Arc<Downloader>) -> Self {
        Self { id: channel_id, downloader }
    }

    pub async fn browse(self) -> Result<ChannelContentBrowse> {
        let resp = api_request(&self.id).await?;
        resp.extract_all_releases(self.downloader)
    }
}
