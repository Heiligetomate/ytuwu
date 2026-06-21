use crate::{Result, error::YtuwuError, id_resolver::id::Id, request::clients::BrowseClient};

use serde::{Deserialize, Serialize};

/// This struct is for normal album ids.
/// Example: OLAK5uy_mgi7GF3ptCZvPbGOBICaqmMQlHCH7p0Uk
/// When creating a new AlbumId, it checks for the correct length and the correct format. An album
/// id should always start with OLAK5uy_
/// After validating, it adds VL at the beginning because thats what youtube expects
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct AlbumId {
    id: String,
}

impl Id for AlbumId {
    type Client = BrowseClient;

    fn new<T: Into<String>>(id: T) -> Result<Self> {
        let raw_id = id.into();

        if raw_id.len() != 41 {
            return Err(YtuwuError::InvalidIdLength);
        }

        if !raw_id.starts_with("OLAK5uy") {
            return Err(YtuwuError::InvalidIdFormat);
        }

        Ok(Self { id: format!("VL{}", raw_id) })
    }

    fn get_id(self) -> String {
        self.id
    }

    fn as_str(&self) -> &str {
        &self.id
    }
}
