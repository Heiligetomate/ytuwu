use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{
        collection::IdCollection,
        id::{GetId, Id},
    },
    request::{api_request, clients::ChannelClient},
    types::{channel_name::ChannelNameId, channel_raw::ChannelRawId},
};

use serde::{Deserialize, Serialize};

/// Enum holding all possible channel ids.
/// This is a wrapper for all Channel ids which is needed for a clean .get_id() in the api.
/// When creating a new ChannelId, it tries to create all different channel ids.
/// This works because all Channel ids have a different format.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ChannelId {
    /// For raw channel ids as they are in the url when browsing an artist (occurs behind /channel).
    Id(ChannelRawId),
    /// For an account name like ntomusic. Does not have a fixed format and therefore will never
    /// fail to create.
    Name(ChannelNameId),
}

impl ChannelId {
    /// Returns a &str that tells the id type that is currently stored in this channel.
    /// Needed for a clean IdCollection info function.
    /// Panics if both channel name and channel id are none because that should never even be
    /// possible, invalid state.
    pub fn info(&self) -> &str {
        match &self {
            Self::Id(_) => "ChannelId",
            Self::Name(_) => "ChannelName",
        }
    }

    // TODO: Maybe make this return the correct channel id?
    /// This function returns self if id is Some.
    /// This function uses the ChannelNameId client to convert the name to an id if name is Some.
    pub async fn make_valid(self, client: &reqwest::Client) -> Result<Self> {
        match self {
            ChannelId::Id(_) => Ok(self),
            ChannelId::Name(name) => {
                let response = api_request(&name, client).await?;
                let id = response.get_id()?;

                Ok(Self::Id(ChannelRawId::new(id)?))
            }
        }
    }
}

impl Id for ChannelId {
    type Client = ChannelClient;

    fn new<T: Into<String>>(id: T) -> Result<Self> {
        let raw_id = id.into();

        if let Ok(id) = ChannelRawId::new(raw_id.as_str()) {
            return Ok(Self::Id(id));
        } else if let Ok(name_id) = ChannelNameId::new(raw_id.as_str()) {
            return Ok(Self::Name(name_id));
        } else {
            return Err(YtuwuError::IdCreationError(Some("Creating a channel id from the raw id was not possible because no format matched".into())));
        }
    }

    fn get_id(self) -> String {
        let extracted = match self {
            ChannelId::Id(id) => id.get_id(),
            ChannelId::Name(name) => name.get_id(),
        };

        extracted
    }

    fn as_str(&self) -> &str {
        let extracted = match self {
            ChannelId::Id(id) => id.as_str(),
            ChannelId::Name(name) => name.as_str(),
        };

        extracted
    }
}

impl GetId<ChannelId> for IdCollection {
    fn get_id(&self) -> Result<ChannelId> {
        Ok(self
            .channel_id
            .clone()
            .ok_or(crate::error::get_id_err("channelId", &self))?)
    }
}
