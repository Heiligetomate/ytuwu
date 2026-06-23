use std::{error::Error, fmt::Display};

use uuid::Uuid;

use crate::id_resolver::IdCollection;

/// Custom restul for ytuwu which should be used for every funciton in this library that returns a
/// result.
/// Holds a generic value T and a variant of the enum YtuwuError
pub type Result<T> = std::result::Result<T, YtuwuError>;

/// This type is for errors and should be used with the fmt_err_inf function to implement Display
/// cleanly.
/// This allows the usage of the errors without forcing an extra error message
type ErrInf = Option<String>;

/// This enum gets used inside the Storage variant in the YtuwuError enum
/// Holds multiple different types of things that can go wrong in the downlaoder storage for
/// downloaded media and media bundles.
#[derive(Debug, Clone)]
pub enum StorageError {
    /// Extraction of the channel template with the given id failed.
    /// Holds the Uuid that was used to extract the template.
    ChannelTemplateExtraction(Uuid),
    /// Extraction of the playlist name with the given id failed.
    /// Holds the Uuid that was used to extract the name.
    ListNameExtraction(Uuid),
    /// Extaction of the media with the given id failed.
    /// Holds the Uuid that was used to extract the media.
    MediaExtraction(Uuid),
}

// TODO: documents this
// TODO: Clean this up
#[derive(Debug, Clone)]
pub enum YtuwuError {
    // File related errors
    //
    /// There was a directory expected but something else like a file path was given.
    NotADir(ErrInf),
    /// A file could not be created.
    CreateFile(ErrInf),
    /// Writing to a file failed.
    WriteToFile(ErrInf),
    /// Failed to create a directory.
    CreateDir(ErrInf),

    // Id related errors
    //
    /// IdCollection did not contain the wanted id.
    IdNotContained(ErrInf),
    /// Creating the id went wrong. This is used for the enums "channelId" and "browseId" in the
    /// IdCollection
    IdCreationError(ErrInf),
    /// The id creation failed because the length was invalid. Holds the id type and the expected
    /// id length.
    InvalidIdLength((&'static str, u16)),
    /// The id creation failed because the format was invalid. Holds the id type and the expected
    /// format
    InvalidIdFormat((&'static str, &'static str)),

    /// Used for any storage related error
    /// Holds a StorageError
    Storage(StorageError),

    BrowseDataNotFound(&'static str),
    PlayerDataNotFound(&'static str),
    ChannelDataNotFound(&'static str),

    ReqwestError(String),
    Deserialize(String),
    Tokio(String),

    UrlParsing(&'static str),

    NoThumbnail,
    NoLowerItagFound,
    NoMatchingStream,
    NoMatchingThumbnail,
    SongInPlaylistNotFound,
    CaptchaBypassFailed(u16),

    YoutubeAPIReturn,

    UrlSizeExtract,
    EmptyMediaBundle,
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

/// Builds a clean error message stating what id was expected and what ids are available
/// Always returns YtuwuError::NoIdFound
/// Example
///```rust
///impl GetId<VideoId> for IdCollection {
///    fn get_id(&self) -> Result<VideoId> {
///        Ok(self
///            .video_id
///            .clone()
///            .ok_or(crate::error::get_id_err("videoId", &self))?)
///    }
///}
///```
pub fn get_id_err(expected_type: &str, id_collection: &IdCollection) -> YtuwuError {
    let existing_ids = id_collection.info();
    let err_string = format!("Failed to get {} from IdCollection. Existing ids in this IdCollection: {}", expected_type, existing_ids);

    YtuwuError::IdNotContained(Some(err_string))
}

impl Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ChannelTemplateExtraction(id) => write!(f, "Channel template with the given id was not found in the channel template collection. Given id: {id}"),
            Self::ListNameExtraction(id) => write!(f, "Playlist name with the given id was not found in the playlist name collection. Given id: {id}"),
            Self::MediaExtraction(id) => write!(f, "Media with the given id was not found in the storage. Given id: {id}"),
        }
    }
}

impl Display for YtuwuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // File related
            Self::NotADir(e) => write!(f, "Expected a dir, did not find a dir{}", fmt_err_inf(e)),
            Self::CreateFile(e) => write!(f, "Failed to create file{}", fmt_err_inf(e)),
            Self::WriteToFile(e) => write!(f, "Failed to write to file{}", fmt_err_inf(e)),
            Self::CreateDir(e) => write!(f, "Failed to create a directory{}", fmt_err_inf(e)),

            // Id cration, extractiom related
            Self::IdNotContained(e) => write!(f, "Id was not found in the IdCollection{}", fmt_err_inf(e)),
            Self::IdCreationError(e) => write!(f, "Id creation failed{}", fmt_err_inf(e)),
            Self::InvalidIdLength((id, len)) => write!(f, "{} has an invalid length. Expected length: {}", id, len),
            Self::InvalidIdFormat((id, frm)) => write!(f, "{} has an invalid format. Expected format: {}", id, frm),

            // Storage related
            Self::Storage(e) => write!(f, "Storage error: {}", e.to_string()),

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
            Self::UrlParsing(e) => write!(f, "Error while parsing the url: {e}"),
            Self::SongInPlaylistNotFound => write!(f, "Song was not found"),
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
