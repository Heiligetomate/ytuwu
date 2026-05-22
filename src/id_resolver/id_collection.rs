use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{channel_id::ChannelId, id::Id, playlist_id::FastBrowseId, short_id::ShortId, video_id::VideoId},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct IdCollection {
    pub(super) video_id: Option<VideoId>,
    pub(super) browse_id: Option<FastBrowseId>,
    pub(super) short_id: Option<ShortId>,
    pub(super) channel_id: Option<ChannelId>,
}

enum Host {
    YoutubeDotCom,
    MusicYoutubeDotCom,
    YoutubeDotBe,
}

impl Host {
    fn parse(raw: &str) -> Result<Self> {
        let host = raw
            .strip_prefix("www.")
            .or_else(|| raw.strip_prefix("m."))
            .unwrap_or(raw);

        match host {
            "youtube.com" | "youtube-nocookie.com" => Ok(Self::YoutubeDotCom),
            "music.youtube.com" => Ok(Self::MusicYoutubeDotCom),
            "youtu.be" => Ok(Self::YoutubeDotBe),
            _ => Err(YtuwuError::UrlParsing("invalid host")),
        }
    }
}

impl IdCollection {
    pub fn is_empty(&self) -> bool {
        return self.video_id.is_none() && self.browse_id.is_none() && self.channel_id.is_none() && self.short_id.is_none();
    }

    pub fn from_url<T: Into<String>>(raw_url: T) -> Result<Self> {
        let url_string: String = raw_url.into();
        let url = Url::parse(&url_string).map_err(|_| YtuwuError::UrlParsing("could not parse the url"))?;

        let host = Host::parse(
            url.host_str()
                .ok_or(YtuwuError::UrlParsing("host not found"))?,
        )?;

        let result = match host {
            Host::YoutubeDotBe => Self::from_youtu_be(&url)?,
            Host::YoutubeDotCom => Self::from_youtube_com(&url)?,
            Host::MusicYoutubeDotCom => Self::from_music_youtube_com(&url)?,
        };

        if result.is_empty() {
            return Err(YtuwuError::UrlParsing("url contains no valid id"));
        }

        Ok(result)
    }

    fn from_youtu_be(url: &Url) -> Result<Self> {
        let id = url
            .path_segments()
            .and_then(|mut s| s.next())
            .filter(|s| !s.is_empty())
            .ok_or(YtuwuError::UrlParsing("no video id in youtu.be url"))?;

        Ok(Self::with_video(VideoId::new(id)))
    }

    fn from_youtube_com(url: &Url) -> Result<Self> {
        let segments = Self::path_segments(url);
        let first = segments.first().copied().unwrap_or("");

        match first {
            "shorts" => {
                let id = Self::id_from_segments(segments).ok_or(YtuwuError::UrlParsing("no short id found"))?;
                Ok(Self::with_short(ShortId::new(id)))
            }

            "embed" | "v" | "e" => {
                let id = Self::id_from_segments(segments).ok_or(YtuwuError::UrlParsing("no video id found"))?;
                Ok(Self::with_video(VideoId::new(id)))
            }

            "channel" => {
                let id = Self::id_from_segments(segments).ok_or(YtuwuError::UrlParsing("no channel id found"))?;
                Ok(Self::with_channel(ChannelId::new(id)))
            }

            s if s.starts_with('@') => Err(YtuwuError::UrlParsing("not suppoerted")),
            "c" | "user" => Err(YtuwuError::UrlParsing("not suppoerted")),

            "watch" | "playlist" | "" => Self::from_query_params(url),

            _ => Err(YtuwuError::UrlParsing("invalid url path")),
        }
    }

    fn from_music_youtube_com(url: &Url) -> Result<Self> {
        let segments = Self::path_segments(url);
        let first = segments.first().copied().unwrap_or("");

        match first {
            "browse" => {
                let id = Self::id_from_segments(segments).ok_or(YtuwuError::UrlParsing("no browse id found"))?;

                if id.starts_with("UC") {
                    Ok(Self::with_channel(ChannelId::new(id)))
                } else if id.starts_with("MPAD") {
                    // idk if that exists
                    panic!("wow didnt know that exists! Please open an issue or smth containing the url you used");
                } else {
                    Ok(Self::with_browse(FastBrowseId::new(id)))
                }
            }

            "channel" => {
                let id = Self::id_from_segments(segments).ok_or(YtuwuError::UrlParsing("no channel id found"))?;
                Ok(Self::with_channel(ChannelId::new(id)))
            }

            s if s.starts_with('@') => Err(YtuwuError::UrlParsing("not implemented yet")),

            "watch" | "" => Self::from_query_params(url),

            _ => Err(YtuwuError::UrlParsing("invalid url path")),
        }
    }

    fn from_query_params(url: &Url) -> Result<Self> {
        let mut result = Self::empty();
        for (key, value) in url.query_pairs() {
            match key.as_ref() {
                "v" => result.video_id = Some(VideoId::new(value.as_ref())),
                "list" => result.browse_id = Some(FastBrowseId::new(value.as_ref())),
                _ => {}
            }
        }
        Ok(result)
    }

    fn path_segments(url: &Url) -> Vec<&str> {
        url.path_segments()
            .map(|s| s.collect())
            .unwrap_or_default()
    }

    fn id_from_segments(segments: Vec<&str>) -> Option<&str> {
        let id = segments
            .get(1)
            .filter(|s| !s.is_empty());
        id.copied()
    }

    fn empty() -> Self {
        Self {
            video_id: None,
            browse_id: None,
            channel_id: None,
            short_id: None,
        }
    }

    fn with_video(id: VideoId) -> Self {
        Self { video_id: Some(id), ..Self::empty() }
    }

    fn with_browse(id: FastBrowseId) -> Self {
        Self { browse_id: Some(id), ..Self::empty() }
    }

    fn with_channel(id: ChannelId) -> Self {
        Self { channel_id: Some(id), ..Self::empty() }
    }

    fn with_short(id: ShortId) -> Self {
        Self { short_id: Some(id), ..Self::empty() }
    }
}
