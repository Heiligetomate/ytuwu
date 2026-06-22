use std::{error::Error, fmt::Display};

/// Custom restul for ytuwu which should be used for every funciton in this library that returns a
/// result.
/// Holds a generic value T and a variant of the enum YtuwuError
pub type Result<T> = std::result::Result<T, YtuwuError>;

/// This type is for errors and should be used with the fmt_err_inf function to implement Display
/// cleanly.
/// This allows the usage of the errors without forcing an extra error message
type ErrInf = Option<String>;

// TODO: documents this
// TODO: Clean this up
#[derive(Debug, Clone)]
pub enum YtuwuError {
    /// File related errors
    ///
    /// There was a directory expected but something else like a file path was given.
    NotADir(ErrInf),
    /// A file could not be created.
    CreateFile(ErrInf),
    /// Writing to a file failed.
    WriteToFile(ErrInf),
    /// Failed to create a directory.
    CreateDir(ErrInf),

    BrowseDataNotFound(&'static str),
    PlayerDataNotFound(&'static str),
    ChannelDataNotFound(&'static str),

    ReqwestError(String),
    Deserialize(String),
    Tokio(String),

    UrlParsing(&'static str),

    MediaNotInStorage,
    NoThumbnail,
    NoLowerItagFound,
    NoMatchingStream,
    NoMatchingThumbnail,
    ListNameNotFound,
    SongInPlaylistNotFound,

    CaptchaBypassFailed(u16),
    YoutubeAPIReturn,

    UrlSizeExtract,
    EmptyMediaBundle,

    NoIdFound,
    InvalidChannelId,
    InvalidIdLength,
    InvalidIdFormat,
}

/// Adds a colon with space infront of the error message if it is Some, returns a . if the error
/// message is None.
/// Use this for implementing Display and do not add a . at the end of the first part of the error
/// message because this would break the format
fn fmt_err_inf(opt_err: &ErrInf) -> String {
    match opt_err {
        Some(e) => format!(": {}", e),
        None => String::from("."),
    }
}

impl Display for YtuwuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotADir(e) => write!(f, "Expected a dir, did not find a dir{}", fmt_err_inf(e)),
            Self::CreateFile(e) => write!(f, "Failed to create file{}", fmt_err_inf(e)),
            Self::WriteToFile(e) => write!(f, "Failed to write to file{}", fmt_err_inf(e)),
            Self::CreateDir(e) => write!(f, "Failed to create a directory{}", fmt_err_inf(e)),

            Self::InvalidChannelId => write!(f, "Channel Id is invalid and was not found"),
            Self::ListNameNotFound => write!(f, "Playlist name was not found"),
            Self::MediaNotInStorage => write!(f, "the media with the id was not found in the downloaded storage"),
            Self::EmptyMediaBundle => write!(f, "media bundle was empty"),
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

impl From<std::io::Error> for YtuwuError {
    fn from(_: std::io::Error) -> Self {
        Self::YoutubeAPIReturn
    }
}

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
