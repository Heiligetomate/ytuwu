use serde::{Deserialize, Serialize};
use std::fmt;
use url::Url;

use crate::{Result, error::YtuwuError};

pub trait GetId<T: Id> {
    fn get_id(&self) -> Result<T>;
}

pub trait Id {
    fn new<T: Into<String>>(id: T) -> Self;
    fn get_id(self) -> String;
    fn as_str(&self) -> &str;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct BrowseId {
    id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct VideoId {
    id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ChannelId {
    id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ShortId {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdCollection {
    video_id: Option<VideoId>,
    browse_id: Option<BrowseId>,
    short_id: Option<ShortId>,
    channel_id: Option<ChannelId>,
}

impl Id for BrowseId {
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

impl Id for VideoId {
    fn new<T: Into<String>>(id: T) -> Self {
        Self { id: id.into() }
    }

    fn get_id(self) -> String {
        self.id
    }

    fn as_str(&self) -> &str {
        &self.id
    }
}

impl Id for ChannelId {
    fn new<T: Into<String>>(id: T) -> Self {
        Self { id: id.into() }
    }

    fn get_id(self) -> String {
        self.id
    }

    fn as_str(&self) -> &str {
        &self.id
    }
}

impl Id for ShortId {
    fn new<T: Into<String>>(id: T) -> Self {
        Self { id: id.into() }
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
            .ok_or(YtuwuError::NoIdFound)?)
    }
}

impl GetId<BrowseId> for IdCollection {
    fn get_id(&self) -> Result<BrowseId> {
        Ok(self
            .browse_id
            .clone()
            .ok_or(YtuwuError::NoIdFound)?)
    }
}

impl GetId<ChannelId> for IdCollection {
    fn get_id(&self) -> Result<ChannelId> {
        Ok(self
            .channel_id
            .clone()
            .ok_or(YtuwuError::NoIdFound)?)
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

impl fmt::Display for IdCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let browse_display: String = { if let Some(id) = &self.browse_id { id.as_str().to_owned() } else { "None".to_owned() } };
        let video_display: String = { if let Some(id) = &self.video_id { id.as_str().to_owned() } else { "None".to_owned() } };
        write!(f, "browse_id: {} \nvideo_id : {}", &browse_display, &video_display)
    }
}

impl IdCollection {
    pub fn from_url<T: Into<String>>(raw_url: T) -> Result<Self> {
        let mut result = Self {
            video_id: None,
            browse_id: None,
            channel_id: None,
            short_id: None,
        };

        let url_string: String = raw_url.into();
        let url: Url = Url::parse(&url_string).map_err(|_| YtuwuError::UrlParsing("could not parse the url"))?;

        match url
            .host_str()
            .ok_or(YtuwuError::UrlParsing("host not found"))?
        {
            "youtu.be" => {}
            "youtube.com" => {}
            "music.youtube.com" => {}
            _ => return Err(YtuwuError::UrlParsing("invalid host")),
        }

        let mut params: Vec<(String, String)> = url
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();

        for param in params.drain(..) {
            match param.0.as_ref() {
                "v" => result.video_id = Some(VideoId::new(param.1)),
                "list" => result.browse_id = Some(BrowseId::new(param.1)),
                _ => {}
            }
        }

        let segments: Option<Vec<&str>> = url
            .path_segments()
            .map(|c| c.collect())
            .ok_or("no")
            .ok();

        if let Some(url_parts) = segments {
            if let Some(first_segment) = url_parts.get(0) {
                match *first_segment {
                    "channel" => {
                        let id = url_parts
                            .get(1)
                            .ok_or(YtuwuError::UrlParsing("no channel id found"))?;
                        result.channel_id = Some(ChannelId::new(*id));
                    }
                    "shorts" => {
                        let id = url_parts
                            .get(1)
                            .ok_or(YtuwuError::UrlParsing("no short id found"))?;
                        result.short_id = Some(ShortId::new(*id));
                    }
                    "media" | "watch" => {}
                    _ => result.short_id = Some(ShortId::new(*first_segment)),
                }
            }
        }

        Ok(result)
    }
}
