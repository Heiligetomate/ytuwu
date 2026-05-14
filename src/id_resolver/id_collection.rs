use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{channel_id::ChannelId, playlist_id::BrowseId, short_id::ShortId, video_id::VideoId},
};

use super::id::Id;

#[derive(Debug, Serialize, Deserialize)]
pub struct IdCollection {
    pub(super) video_id: Option<VideoId>,
    pub(super) browse_id: Option<BrowseId>,
    pub(super) short_id: Option<ShortId>,
    pub(super) channel_id: Option<ChannelId>,
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
