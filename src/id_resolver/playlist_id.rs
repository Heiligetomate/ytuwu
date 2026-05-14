use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{
        id::{GetId, Id},
        id_collection::IdCollection,
    },
    request::clients::browse::BrowseClient,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct BrowseId {
    id: String,
}

impl Id for BrowseId {
    type Client = BrowseClient;

    fn new<T: Into<String>>(id: T) -> Self {
        Self { id: format!("VL{}", id.into()) }
    }

    fn get_id(self) -> String {
        self.id
    }

    fn as_str(&self) -> &str {
        &self.id
    }
}

impl GetId<BrowseId> for IdCollection {
    fn get_id(&self) -> Result<BrowseId> {
        Ok(self
            .browse_id
            .clone()
            .ok_or(YtuwuError::NoIdFound)?)
    }
}
