use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{
        collection::IdCollection,
        id::{GetId, Id},
    },
    request::clients::PlayerClient,
};

use serde::{Deserialize, Serialize};

/// This id is used for downloading media by using the player client
/// When creating a new VideoId, it checks if the len is 11 and if there are any forbidden
/// characters in the raw id
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct VideoId {
    id: String,
}

impl Id for VideoId {
    type Client = PlayerClient;

    fn new<T: Into<String>>(id: T) -> Result<Self> {
        let raw_id = id.into();
        if raw_id.len() != 11 {
            return Err(YtuwuError::InvalidIdLength(("VideoId", 11)));
        }
        if !raw_id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(YtuwuError::InvalidIdFormat(("VideoId", "Can not contain special characters")));
        }

        Ok(Self { id: raw_id })
    }

    fn get_id(self) -> String {
        self.id
    }

    fn as_str(&self) -> &str {
        &self.id
    }
}

impl GetId<VideoId> for IdCollection {
    fn get_id(&self) -> Result<VideoId> {
        Ok(self
            .video_id
            .clone()
            .ok_or(crate::error::get_id_err("videoId", &self))?)
    }
}
