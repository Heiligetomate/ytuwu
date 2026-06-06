use std::{error::Error, fmt::Display};

pub type Result<T> = std::result::Result<T, YtuwuError>;

#[derive(Debug, Clone)]
pub enum YtuwuError {
    BrowseDataNotFound(&'static str),
    PlayerDataNotFound(&'static str),
    ChannelDataNotFound(&'static str),
    ReqwestError(String),
    CaptchaBypassFailed(u16),

    NoThumbnail,

    YoutubeAPIReturn,
    Deserialize(String),

    NoLowerItagFound,
    NoMatchingStream,

    NoMatchingThumbnail,

    UrlSizeExtract,
    EmptyMediaBundle,
    CreateFile,
    CreateDir,
    WriteToFile,
    InvalidPath,
    NoIdFound,
    SongInPlaylistNotFound,

    UrlParsing(&'static str),
    InvalidIdLength,
    InvalidIdFormat,

    ProgressHandler,

    Tokio(String),
}

impl Display for YtuwuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyMediaBundle => write!(f, "media bundle was empty"),
            Self::ProgressHandler => write!(f, "Error while getting progress handler"),
            Self::Tokio(e) => write!(f, "tokio error: {}", e),
            Self::BrowseDataNotFound(e) => write!(f, "Could not get data from response: {}.", e),
            Self::PlayerDataNotFound(e) => {
                write!(f, "Could not get data from player response: {}.", e)
            }
            Self::ChannelDataNotFound(e) => write!(f, "Could not get data from response: {}", e),
            Self::ReqwestError(e) => write!(f, "Reqwest failed: {e}"),
            Self::CaptchaBypassFailed(e) => {
                write!(f, "The captcha bypass failed after {} tries.", e)
            }
            Self::YoutubeAPIReturn => write!(f, "Youtube API gave an unexpected reply."),
            Self::Deserialize(e) => write!(f, "Could not deserialize the response. {e}"),
            Self::NoLowerItagFound => write!(f, "Could not find any lower itag."),
            Self::NoMatchingStream => write!(f, "No matching stream found for this itag."),
            Self::UrlSizeExtract => write!(f, "Failed to extract the size from the url."),
            Self::CreateFile => write!(f, "Failed to create the file."),
            Self::CreateDir => write!(f, "Failed to create the dir."),
            Self::WriteToFile => write!(f, "Failed to write bytes to the file."),
            Self::InvalidPath => write!(f, "Invalid path format."),
            &Self::NoIdFound => write!(f, "Could not get id from collection"),
            Self::UrlParsing(e) => write!(f, "Error while parsing the url: {e}"),
            Self::SongInPlaylistNotFound => write!(f, "Song was not found"),
            Self::InvalidIdLength => write!(f, "Id has an invalid length"),
            Self::InvalidIdFormat => write!(f, "Id has an invalid format"),
            Self::NoMatchingThumbnail => write!(f, "No matching thumbnail found"),
            Self::NoThumbnail => write!(f, "No Thumbnail"),
        }
    }
}

impl Error for YtuwuError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

// impl From<std::io::Error> for YtuwuError {
//     fn from(_: std::io::Error) -> Self {
//         Self::YoutubeAPIReturn
//     }
// }

impl From<serde_json::Error> for YtuwuError {
    fn from(value: serde_json::Error) -> Self {
        Self::Deserialize(value.to_string())
    }
}

impl From<reqwest::Error> for YtuwuError {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value.to_string())
    }
}

impl From<tokio::task::JoinError> for YtuwuError {
    fn from(value: tokio::task::JoinError) -> Self {
        Self::Tokio(value.to_string())
    }
}

impl From<tokio::sync::AcquireError> for YtuwuError {
    fn from(value: tokio::sync::AcquireError) -> Self {
        Self::Tokio(value.to_string())
    }
}
