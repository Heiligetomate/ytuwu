use std::sync::Arc;

use uuid::Uuid;

use crate::{
    Result,
    downloader::{Downloader, channel::ChannelContentBrowse},
    id_resolver::types::ChannelId,
    request::api_request,
};

/// This struct is the first step for downloading a channel.
/// It holds a ChannelId, an arc downloader for
/// shared data and it holds an uuid for identification
pub struct ChannelBrowse {
    channel_id: ChannelId,
    downloader: Arc<Downloader>,
    id: Uuid,
}

impl ChannelBrowse {
    /// Creates a new ChannelBrowse by first validatiing the channel id which converts the channel
    /// id to a channel containing only a valid id and not a name
    /// Creates a new Uuid if there was none supplied
    /// Failes if the conversion of the channel id failed which can happen when there was a wrong
    /// name supplied.
    pub async fn new(channel_id: ChannelId, downloader: Arc<Downloader>, id: Option<Uuid>) -> Result<Self> {
        let id = id.unwrap_or(Uuid::new_v4());

        let channel_id = channel_id
            .make_valid(&downloader.client)
            .await?;

        Ok(Self { channel_id, downloader, id })
    }

    /// Consumes itself and makes a request with the channel id
    /// After that it extracts all releases from that Response
    /// Fails if the request went wrong ot the extraction did not work
    pub async fn browse(self) -> Result<ChannelContentBrowse> {
        let resp = api_request(&self.channel_id, &self.downloader.client).await?;
        resp.extract_all_releases(self.downloader, self.id)
    }
}
