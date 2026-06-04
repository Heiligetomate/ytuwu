use crate::{Result, error::YtuwuError, id_resolver::id::Id, request::clients::playlist::PlaylistBrowseClient};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PlaylistId {
    id: String,
}

impl Id for PlaylistId {
    type Client = PlaylistBrowseClient;

    fn new<T: Into<String>>(id: T) -> Result<Self> {
        let raw_id = id.into();

        match raw_id.len() {
            43 => {
                if !raw_id.starts_with("RDCLAK5uy") {
                    return Err(YtuwuError::InvalidIdFormat);
                }
            }
            34 => {
                if !raw_id.starts_with("PL") {
                    return Err(YtuwuError::InvalidIdFormat);
                }
            }
            _ => return Err(YtuwuError::InvalidIdLength),
        };

        Ok(Self { id: format!("VL{}", raw_id) })
    }

    fn get_id(self) -> String {
        self.id
    }

    fn as_str(&self) -> &str {
        &self.id
    }
}
