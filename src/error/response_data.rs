use std::fmt::Display;

/// Used for any missing response data
/// Every client has its own variant for better overview
#[derive(Debug, Clone)]
pub enum ResponseDataError {
    /// Used when there was a part of the channel browse response missing
    /// Holds the data that was expected but missing
    ChannelBrowse(&'static str),

    /// Used when there was a part of the channel name conversion response missing
    /// Holds the data that was expected but missing
    ChannelName(&'static str),

    /// Used when there was a part of the fast browse response missing
    /// Holds the data that was expected but missing
    FastBrowse(&'static str),

    /// Used when there was a part of the player response missing
    /// Holds the data that was expected but missing
    Player(&'static str),

    /// Used when there was a part of the playlist browse response missing
    /// Holds the data that was expected but missing
    Playlist(&'static str),

    /// Used when there was a part of the slow browse response missing
    /// Holds the data that was expected but missing
    SlowBrowse(&'static str),
}

impl Display for ResponseDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseDataError::ChannelBrowse(e) => write!(f, "Channel browse data missing: {}", e),
            ResponseDataError::ChannelName(e) => write!(f, "Channel Name conversion data missing: {}", e),
            ResponseDataError::FastBrowse(e) => write!(f, "Fast browsing data missing: {}", e),
            ResponseDataError::Player(e) => write!(f, "Player data missing: {}", e),
            ResponseDataError::Playlist(e) => write!(f, "Playlist data missing: {}", e),
            ResponseDataError::SlowBrowse(e) => write!(f, "Slow browsing data missing: {}", e),
        }
    }
}
