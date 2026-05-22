use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{
        id::{GetId, Id, MakeChannelId},
        id_collection::IdCollection,
    },
    request::clients::channel::ChannelClient,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ChannelId {
    id: String,
}

impl Id for ChannelId {
    type Client = ChannelClient;

    fn new<T: Into<String>>(id: T) -> Self {
        Self { id: id.into() }
    }

    fn get_id(self) -> String {
        self.id
    }

    fn as_str(&self) -> &str {
        &self.id
    }
}

impl GetId<ChannelId> for IdCollection {
    fn get_id(&self) -> Result<ChannelId> {
        Ok(self
            .channel_id
            .clone()
            .ok_or(YtuwuError::NoIdFound)?)
    }
}

impl MakeChannelId for ChannelId {
    async fn transform(&self) -> Result<ChannelId> {
        Ok(self.clone())
    }
}
