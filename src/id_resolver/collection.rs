use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{
        id::Id,
        types::{AlbumId, BrowseId, ChannelId, ShortId, VideoId},
    },
    types::PlaylistId,
};

/// An IdCollection hold all different Id types which are not publically exposed.
/// Getting the ids is achieved by calling get_id() which should automatically return the correct
/// type
#[derive(Debug, Serialize, Deserialize)]
pub struct IdCollection {
    /// regular video id
    pub(super) video_id: Option<VideoId>,
    /// video id but as a short id
    pub(super) short_id: Option<ShortId>,
    /// channel id struct holding either name or id
    pub(super) channel_id: Option<ChannelId>,
    /// browse id enum holding one browse id
    pub(super) browse_id: Option<BrowseId>,
}

/// Enum for the different youtube hosts
enum Host {
    /// www.youtube.com
    YoutubeDotCom,
    /// music.youtube.com
    MusicYoutubeDotCom,
    /// www.youtu.be
    YoutubeDotBe,
}

impl Host {
    /// takes the raw url and tries to parse it to one of the varients
    /// Returns Err if the host is invalid
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
    /// Goes through all ids on the id collection and checks if the id is Some.
    /// Returns a String containing the name of all existing id types
    /// Seperates them with a comma
    /// Good for debugging and clean error messages
    pub fn info(&self) -> String {
        let mut existing_ids = Vec::new();

        if self.video_id.is_some() {
            existing_ids.push("videoId");
        }

        if self.short_id.is_some() {
            existing_ids.push("shortId");
        }

        if let Some(browse_id) = &self.browse_id {
            existing_ids.push(match browse_id {
                BrowseId::AlbumId(_) => "albumId",
                BrowseId::PlaylistId(_) => "playlistId",
                BrowseId::ChannelBrowseId(_) => "channelBrowseId",
            });
        }

        if let Some(channel_id) = &self.channel_id {
            existing_ids.push(channel_id.info());
        }

        let mut info_string = String::new();

        for (i, id) in existing_ids.iter().enumerate() {
            info_string.push_str(id);
            if i == info_string.len() - 1 {
                continue;
            }
            info_string.push_str(", ");
        }

        info_string
    }
    /// returns true if all values of self are None
    /// returns false if any value of self is Some()
    pub fn is_empty(&self) -> bool {
        return self.video_id.is_none() && self.browse_id.is_none() && self.channel_id.is_none() && self.short_id.is_none();
    }

    /// Takes an url, extracts all ids from it and returns an IdCollection containing the ids
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

    /// This gets the video id from any youtu.be url.
    /// This could be a short id but its impossible to know wether its a normal id or a short id
    /// Returns an IdCollection containing the video ID
    /// Fails if the Video id could not be created or if the parsing of the url failed
    fn from_youtu_be(url: &Url) -> Result<Self> {
        let id = url
            .path_segments()
            .and_then(|mut s| s.next())
            .filter(|s| !s.is_empty())
            .ok_or(YtuwuError::UrlParsing("no video id in youtu.be url"))?;

        Ok(Self::with_video(VideoId::new(id)?))
    }

    /// This function handles any youtube.com url and returns an IdCollection containing all
    /// extracted Ids.
    /// It handles /shorts, /embed, /v, /e, /channel, /c, /user, /watch, /playlist and channel names
    /// Returns Err if either the creation of the Id failed or if there was nothing that matched
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

    /// Handles music.youtube.com urls
    /// Extracts all Ids and returns an IdCollection containing the ids
    /// Handles /browse /channel /watch /playlist and channel names
    /// Returns Err if either the creation of the Id failed or if there was nothing that matched
    fn from_music_youtube_com(url: &Url) -> Result<Self> {
        let segments = Self::path_segments(url);
        let first = segments.first().copied().unwrap_or("");

        match first {
            "browse" => {
                let id = Self::id_from_segments(segments, 1).ok_or(YtuwuError::UrlParsing("no browse id found"))?;
                Ok(Self::handle_browse(id)?)
            }

            "channel" => {
                let id = Self::id_from_segments(segments, 1).ok_or(YtuwuError::UrlParsing("no channel id found"))?;
                Ok(Self::with_channel(ChannelId::new(id)?))
            }

            s if s.starts_with('@') => {
                let id = Self::id_from_segments(segments, 0).ok_or(YtuwuError::UrlParsing("no channel id name found"))?;
                Ok(Self::with_channel(ChannelId::new(id)?))
            }

            "watch" | "playlist" | "" => Self::from_query_params(url),

            _ => Err(YtuwuError::UrlParsing("invalid url path")),
        }
    }

    /// Takes an url and returns an IdCollection containing the extracted Ids
    /// handles both ?v and ?list
    /// This function exists because there are urls containing both a video id and a playlist id
    /// which is handled here
    /// example: https://music.youtube.com/watch?v=HPG7gYoqpHM&list=OLAK5uy_mgi7GF3ptCZvPbGOBICaqmMQlHCH7p0Uk
    /// Returns Err if the creation of the Id
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

    /// Get the urls path segemnts as a vec of &str
    fn path_segments(url: &Url) -> Vec<&str> {
        url.path_segments()
            .map(|s| s.collect())
            .unwrap_or_default()
    }

    /// Takes the url segments and an index and returns the segment at that index
    fn id_from_segments(segments: Vec<&str>, index: usize) -> Option<&str> {
        let id = segments
            .get(index)
            .filter(|s| !s.is_empty());
        id.copied()
    }

    /// Returns an IdCollection where every field is None
    /// This is needed as a "filler"
    fn empty() -> Self {
        Self {
            video_id: None,
            channel_id: None,
            browse_id: None,
            short_id: None,
        }
    }

    // TODO: Cant we use BrowseId::new() here?
    /// Inserts a browse id in self
    /// Checks the start of the id and creates the correct Browse id
    /// Returns Err if id failed to create
    fn change_browses(&mut self, raw: &str) -> Result<()> {
        if raw.starts_with("RDCLAK5uy") || raw.starts_with("PL") {
            self.browse_id = Some(BrowseId::PlaylistId(PlaylistId::new(raw)?))
        } else if raw.starts_with("OLAK5uy") {
            self.browse_id = Some(BrowseId::AlbumId(AlbumId::new(raw)?))
        }

        Ok(())
    }

    /// Takes a raw id and returns an IdCollection with the correct BrowseId
    /// Returns Err if id failed to create
    fn handle_browse(id: &str) -> Result<Self> {
        if id.starts_with("UC") || id.starts_with("MPADUC") {
            Ok(Self::with_channel(ChannelId::new(id)?))
        } else if id.starts_with("RDCLAK5uy") || id.starts_with("PL") {
            Ok(Self::with_browse(BrowseId::PlaylistId(PlaylistId::new(id)?)))
        } else if id.starts_with("OLAK5uy") {
            Ok(Self::with_browse(BrowseId::AlbumId(AlbumId::new(id)?)))
        } else {
            Err(YtuwuError::NoIdFound)
        }
    }

    /// Creates a new IdCollection with just a videoId
    fn with_video(id: VideoId) -> Self {
        Self { video_id: Some(id), ..Self::empty() }
    }

    /// Creates a new IdCollection with just a browseId
    fn with_browse(id: BrowseId) -> Self {
        Self { browse_id: Some(id), ..Self::empty() }
    }

    /// Creates a new IdCollection with just a channelId
    fn with_channel(id: ChannelId) -> Self {
        Self { channel_id: Some(id), ..Self::empty() }
    }

    /// Creates a new IdCollection with just a shortId
    fn with_short(id: ShortId) -> Self {
        Self { short_id: Some(id), ..Self::empty() }
    }
}
