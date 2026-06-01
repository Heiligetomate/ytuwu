use serde::{Deserialize, Serialize};

use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{
        collection::IdCollection,
        id::{GetId, Id, MakeChannelId},
        types::ChannelId,
    },
    request::{clients::channel_name_to_id::ChannelNameClient, core::api_request},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ChannelNameId {
    name: String,
}

impl Id for ChannelNameId {
    type Client = ChannelNameClient;

    fn new<T: Into<String>>(id: T) -> Result<Self> {
        let name = id.into();
        let name = name
            .strip_prefix('@')
            .unwrap_or(&name)
            .to_string();
        Ok(Self { name })
    }

    fn get_id(self) -> String {
        self.name
    }

    fn as_str(&self) -> &str {
        &self.name
    }
}

impl GetId<ChannelNameId> for IdCollection {
    fn get_id(&self) -> Result<ChannelNameId> {
        Ok(self
            .channel_name
            .clone()
            .ok_or(YtuwuError::NoIdFound)?)
    }
}

impl MakeChannelId for ChannelNameId {
    async fn transform(&self, client: &reqwest::Client) -> Result<ChannelId> {
        let response = api_request(self, client).await?;
        response.get_id()
    }
}
