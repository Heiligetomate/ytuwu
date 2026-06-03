use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{
        collection::IdCollection,
        id::{GetId, Id},
    },
    request::{clients::channel::ChannelClient, core::api_request},
    types::channel_name::ChannelNameId,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ChannelId {
    id: Option<String>,
    name: Option<ChannelNameId>,
}

impl ChannelId {
    pub async fn make_valid(self, client: &reqwest::Client) -> Result<Self> {
        let id = if let Some(id) = self.id {
            id
        } else if let Some(name) = self.name {
            let response = api_request(&name, client).await?;
            response.get_id()?.to_owned()
        } else {
            panic!("Channel id did not contain anything. Invalid state");
        };

        Ok(Self { id: Some(id.to_owned()), name: None })
    }
}

impl Id for ChannelId {
    type Client = ChannelClient;

    fn new<T: Into<String>>(id: T) -> Result<Self> {
        let raw_id = id.into();

        // we need this so that we can get the correct id (the normal uc id is not really compatible
        // with the albums singles and eps we want to extract)
        if raw_id.starts_with("MPADUC") || raw_id.starts_with("UC") {
            let id_with_prfx = {
                if raw_id.starts_with("MPADUC") {
                    raw_id
                } else if raw_id.starts_with("UC") {
                    format!("MPAD{}", raw_id)
                } else {
                    return Err(YtuwuError::InvalidIdFormat);
                }
            };

            if id_with_prfx.len() != 28 {
                return Err(YtuwuError::InvalidIdLength);
            }

            Ok(Self { id: Some(id_with_prfx), name: None })
        } else {
            Ok(Self {
                id: None,
                name: Some(ChannelNameId::new(raw_id)?),
            })
        }
    }

    fn get_id(self) -> String {
        let extracted = if let Some(id) = self.id {
            id
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
