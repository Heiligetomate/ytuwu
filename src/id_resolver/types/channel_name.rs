use serde::{Deserialize, Serialize};

use crate::{Result, id_resolver::id::Id, request::clients::ChannelNameClient};

/// This struct contains a channel name
/// This will later be converted to an id
/// There is no validation when creating a channelname id because there is no format
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ChannelNameId {
    name: String,
}

impl Id for ChannelNameId {
    type Client = ChannelNameClient;

    /// This function nevery fails beecause channel names do not have a fixed format that can be
    /// checked. Therefore, downloading a channel with an invalid ChannelNameId will return Err as
    /// soon as the channel is browsed and not as soon as the id gets created
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
