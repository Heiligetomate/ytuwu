use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ResponseData {
    ChannelBrowse(&'static str),
    ChannelName(&'static str),
    FastBrowse(&'static str),
    Player(&'static str),
    Playlist(&'static str),
    SlowBrowse(&'static str),
}

impl Display for ResponseData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseData::ChannelBrowse(e) => write!(f, "Channel browse data missing: {}", e),
            ResponseData::ChannelName(e) => write!(f, "Channel Name conversion data missing: {}", e),
            ResponseData::FastBrowse(e) => write!(f, "Fast browsing data missing: {}", e),
            ResponseData::Player(e) => write!(f, "Player data missing: {}", e),
            ResponseData::Playlist(e) => write!(f, "Playlist data missing: {}", e),
            ResponseData::SlowBrowse(e) => write!(f, "Slow browsing data missing: {}", e),
        }
    }
}
