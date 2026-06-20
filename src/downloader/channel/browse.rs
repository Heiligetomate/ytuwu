use std::sync::Arc;

use uuid::Uuid;

use crate::{
    Result,
    downloader::{Downloader, channel::ChannelContentBrowse},
    id_resolver::types::ChannelId,
    request::api_request,
};

pub struct ChannelBrowse {
    channel_id: ChannelId,
    downloader: Arc<Downloader>,
    id: Uuid,
}

impl ChannelBrowse {
    pub async fn new(channel_id: ChannelId, downloader: Arc<Downloader>, id: Option<Uuid>) -> Result<Self> {
        let id = id.unwrap_or(Uuid::new_v4());

        let channel_id = channel_id
            .make_valid(&downloader.client)
            .await?;

        Ok(Self { channel_id, downloader, id })
    }

    pub async fn browse(self) -> Result<ChannelContentBrowse> {
        let resp = api_request(&self.channel_id, &self.downloader.client).await?;
        resp.extract_all_releases(self.downloader, self.id)
    }
}
