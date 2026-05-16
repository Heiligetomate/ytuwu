use serde::{Deserialize, Serialize};

use crate::{id_resolver::id::Id, request::clients::browse::BrowseClient};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ChannelPlaylistId {
    id: String,
}

impl Id for ChannelPlaylistId {
    type Client = BrowseClient; /////// NONASDOJASD IOAJD OIJASDO IJAOSDJI OAISDJ NOOOOO WRONG
    ///CLIENT

    fn get_id(self) -> String {
        self.id
    }

    fn as_str(&self) -> &str {
        &self.id
    }

    fn new<T: Into<String>>(id: T) -> Self {
        Self { id: id.into() }
    }
}
