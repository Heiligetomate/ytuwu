use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{
        id::{GetId, Id},
        id_collection::IdCollection,
    },
    request::clients::player::PlayerClient,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ShortId {
    id: String,
}

impl Id for ShortId {
    type Client = PlayerClient;

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

impl GetId<ShortId> for IdCollection {
    fn get_id(&self) -> Result<ShortId> {
        Ok(self
            .short_id
            .clone()
            .ok_or(YtuwuError::NoIdFound)?)
    }
}
