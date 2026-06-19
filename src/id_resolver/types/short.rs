use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{
        collection::IdCollection,
        id::{GetId, Id},
    },
    request::clients::PlayerClient,
    types::VideoId,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ShortId {
    id: String,
}

impl Id for ShortId {
    type Client = PlayerClient;

    fn new<T: Into<String>>(id: T) -> Result<Self> {
        let video_id = VideoId::new(id)?;
        Ok(Self { id: video_id.get_id() })
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

impl ShortId {
    pub fn transform(self) -> Result<VideoId> {
        VideoId::new(self.get_id())
    }
}
