use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{
        id::{BrowseId, GetId, Id},
        id_collection::IdCollection,
    },
    models::fast_browse::FastBrowseResponse,
    request::clients::browse::BrowseClient,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct FastBrowseId {
    id: String,
}

impl Id for FastBrowseId {
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

impl GetId<FastBrowseId> for IdCollection {
    fn get_id(&self) -> Result<FastBrowseId> {
        Ok(self
            .browse_id
            .clone()
            .ok_or(YtuwuError::NoIdFound)?)
    }
}

impl BrowseId for FastBrowseId {
    type BrowseResponse = FastBrowseResponse;
}
