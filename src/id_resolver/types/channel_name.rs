use serde::{Deserialize, Serialize};

use crate::{Result, id_resolver::id::Id, request::clients::channel_name_to_id::ChannelNameClient};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ChannelNameId {
    name: String,
}

impl Id for ChannelNameId {
    type Client = ChannelNameClient;

    fn new<T: Into<String>>(id: T) -> Result<Self> {
        let name = id.into();
        let name = name
            .strip_prefix('@')
            .unwrap_or(&name)
            .to_string();
        Ok(Self { name })
    }

    fn get_id(self) -> String {
        self.name
    }

    fn as_str(&self) -> &str {
        &self.name
    }
}
