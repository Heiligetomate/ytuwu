use serde::{Deserialize, Serialize};

use crate::{Result, error::YtuwuError, id_resolver::id::Id, request::clients::ChannelClient};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ChannelRawId {
    id: String,
}

impl Id for ChannelRawId {
    type Client = ChannelClient;

    // we need this so that we can get the correct id (the normal uc id is not really compatible
    // with the albums singles and eps we want to extract)
    fn new<T: Into<String>>(id: T) -> Result<Self> {
        let raw_id = id.into();
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
