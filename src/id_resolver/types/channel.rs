use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{
        collection::IdCollection,
        id::{GetId, Id},
    },
    request::{clients::channel::ChannelClient, core::api_request},
    types::{channel_name::ChannelNameId, channel_raw::ChannelRawId},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ChannelId {
    id: Option<ChannelRawId>,
    name: Option<ChannelNameId>,
}

impl ChannelId {
    pub async fn make_valid(self, client: &reqwest::Client) -> Result<Self> {
        let id = if let Some(id) = self.id {
            id.get_id()
        } else if let Some(name) = self.name {
            let response = api_request(&name, client).await?;
            response.get_id()?.to_owned()
        } else {
            panic!("Channel id did not contain anything. Invalid state");
        };

        Ok(Self {
            id: Some(ChannelRawId::new(id)?),
            name: None,
        })
    }
}

impl Id for ChannelId {
    type Client = ChannelClient;

    fn new<T: Into<String>>(id: T) -> Result<Self> {
        let raw_id = id.into();
        if let Ok(id) = ChannelRawId::new(raw_id.as_str()) {
            return Ok(Self { id: Some(id), name: None });
        } else if let Ok(name_id) = ChannelNameId::new(raw_id.as_str()) {
            return Ok(Self { id: None, name: Some(name_id) });
        } else {
            return Err(YtuwuError::NoIdFound);
        }
    }

    fn get_id(self) -> String {
        let extracted = if let Some(id) = self.id {
            id.get_id()
        } else if let Some(name) = self.name {
            name.get_id()
        } else {
            panic!("Channel id did not contain anything. Invalid state");
        };
        extracted
    }

    fn as_str(&self) -> &str {
        let extracted = if let Some(id) = self.id.as_ref() {
            id.as_str()
        } else if let Some(name) = self.name.as_ref() {
            name.as_str()
        } else {
            panic!("Channel id did not contain anything. Invalid state");
        };
        extracted
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
