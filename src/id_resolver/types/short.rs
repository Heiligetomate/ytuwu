use crate::{
    Result,
    id_resolver::{
        collection::IdCollection,
        id::{GetId, Id},
    },
    request::clients::PlayerClient,
    types::VideoId,
};

use serde::{Deserialize, Serialize};

// TODO: This is probably not useful
/// This id is for downloading short videos
/// This id is a wrapper for VideoId and the only difference is that when extracting the ids, the
/// short ids can be verified
/// When creating a new ShortId, it uses the same validation as a normal video id
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
            .ok_or(crate::error::get_id_err("shortId", &self))?)
    }
}

impl ShortId {
    pub fn transform(self) -> Result<VideoId> {
        VideoId::new(self.get_id())
    }
}
