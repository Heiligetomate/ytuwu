use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{
        id::Id,
        types::{AlbumId, ChannelId, ChannelNameId, ShortId, VideoId},
    },
    types::PlaylistId,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct IdCollection {
    pub(super) video_id: Option<VideoId>,
    pub(super) album_id: Option<AlbumId>,
    pub(super) playlist_id: Option<PlaylistId>,
    pub(super) short_id: Option<ShortId>,
    pub(super) channel_id: Option<ChannelId>,
    pub(super) channel_name: Option<ChannelNameId>,
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
        return self.video_id.is_none() && self.album_id.is_none() && self.channel_id.is_none() && self.short_id.is_none() && self.channel_name.is_none() && self.playlist_id.is_none();
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

        Ok(Self::with_video(VideoId::new(id)?))
    }

    fn from_youtube_com(url: &Url) -> Result<Self> {
        let segments = Self::path_segments(url);
        let first = segments.first().copied().unwrap_or("");

        match first {
            "shorts" => {
                let id = Self::id_from_segments(segments, 1).ok_or(YtuwuError::UrlParsing("no short id found"))?;
                Ok(Self::with_short(ShortId::new(id)?))
            }

            "embed" | "v" | "e" => {
                let id = Self::id_from_segments(segments, 1).ok_or(YtuwuError::UrlParsing("no video id found"))?;
                Ok(Self::with_video(VideoId::new(id)?))
            }

            "channel" => {
                let id = Self::id_from_segments(segments, 1).ok_or(YtuwuError::UrlParsing("no channel id found"))?;
                Ok(Self::with_channel(ChannelId::new(id)?))
            }

            s if s.starts_with('@') => {
                let id = Self::id_from_segments(segments, 0).ok_or(YtuwuError::UrlParsing("no channel id found"))?;
                Ok(Self::with_channel(ChannelId::new(id)?))
            }
            "c" | "user" => {
                let id = Self::id_from_segments(segments, 1).ok_or(YtuwuError::UrlParsing("no channel id found"))?;
                Ok(Self::with_channel(ChannelId::new(id)?))
            }

            "watch" | "playlist" | "" => Self::from_query_params(url),

            _ => Err(YtuwuError::UrlParsing("invalid url path")),
        }
    }

    fn from_music_youtube_com(url: &Url) -> Result<Self> {
        let segments = Self::path_segments(url);
        let first = segments.first().copied().unwrap_or("");

        match first {
            "browse" => {
                let id = Self::id_from_segments(segments, 1).ok_or(YtuwuError::UrlParsing("no browse id found"))?;
                Ok(Self::with_browse(id)?)
            }

            "channel" => {
                let id = Self::id_from_segments(segments, 1).ok_or(YtuwuError::UrlParsing("no channel id found"))?;
                Ok(Self::with_channel(ChannelId::new(id)?))
            }

            s if s.starts_with('@') => {
                let id = Self::id_from_segments(segments, 0).ok_or(YtuwuError::UrlParsing("no channel id name found"))?;
                Ok(Self::with_channel_name(ChannelNameId::new(id)?))
            }

            "watch" | "playlist" | "" => Self::from_query_params(url),

            _ => Err(YtuwuError::UrlParsing("invalid url path")),
        }
    }

    fn from_query_params(url: &Url) -> Result<Self> {
        let mut result = Self::empty();
        for (key, value) in url.query_pairs() {
            match key.as_ref() {
                "v" => result.video_id = Some(VideoId::new(value.as_ref())?),
                "list" => Self::change_browses(&mut result, &value)?,
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

    fn id_from_segments(segments: Vec<&str>, index: usize) -> Option<&str> {
        let id = segments
            .get(index)
            .filter(|s| !s.is_empty());
        id.copied()
    }

    fn empty() -> Self {
        Self {
            video_id: None,
            album_id: None,
            channel_id: None,
            short_id: None,
            channel_name: None,
            playlist_id: None,
        }
    }

    fn change_browses(&mut self, raw: &str) -> Result<()> {
        if raw.starts_with("RDCLAK5uy") {
            self.playlist_id = Some(PlaylistId::new(raw)?)
        } else if raw.starts_with("OLAK5uy") {
            self.album_id = Some(AlbumId::new(raw)?)
        }

        Ok(())
    }

    fn with_browse(id: &str) -> Result<Self> {
        if id.starts_with("UC") || id.starts_with("MPADUC") {
            Ok(Self::with_channel(ChannelId::new(id)?))
        } else if id.starts_with("RDCLAK5uy") {
            Ok(Self::with_playlist(PlaylistId::new(id)?))
        } else if id.starts_with("OLAK5uy") {
            Ok(Self::with_album(AlbumId::new(id)?))
        } else {
            Err(YtuwuError::NoIdFound)
        }
    }

    fn with_video(id: VideoId) -> Self {
        Self { video_id: Some(id), ..Self::empty() }
    }

    fn with_album(id: AlbumId) -> Self {
        Self { album_id: Some(id), ..Self::empty() }
    }

    fn with_playlist(id: PlaylistId) -> Self {
        Self {
            playlist_id: Some(id),
            ..Self::empty()
        }
    }

    fn with_channel(id: ChannelId) -> Self {
        Self {
            channel_id: Some(id),
            ..Self::empty()
        }
    }

    fn with_short(id: ShortId) -> Self {
        Self { short_id: Some(id), ..Self::empty() }
    }

    fn with_channel_name(id: ChannelNameId) -> Self {
        Self {
            channel_name: Some(id),
            ..Self::empty()
        }
    }
}
