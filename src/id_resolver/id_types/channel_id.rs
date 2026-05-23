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

    fn new<T: Into<String>>(id: T) -> Result<Self> {
        let raw_id = id.into();

        // we need this so that we can get the correct id (the normal uc id is not really compatible
        // with the albums singles and eps we want to extract)
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

        Ok(Self { id: id_with_prfx })
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
