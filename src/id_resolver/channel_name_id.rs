use serde::{Deserialize, Serialize};

use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{
        id::{GetId, Id},
        id_collection::IdCollection,
    },
    request::clients::channel_name_to_id::ChannelNameClient,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ChannelNameId {
    name: String,
}

impl Id for ChannelNameId {
    type Client = ChannelNameClient;

    fn new<T: Into<String>>(id: T) -> Self {
        Self { name: id.into() }
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
