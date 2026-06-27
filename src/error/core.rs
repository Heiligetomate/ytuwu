use crate::error::{ErrInf, ResponseDataError, StorageError, helper::fmt_err_inf};
use std::{error::Error, fmt::Display};

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

    // Url related stuff
    //
    /// Used for url parsing related errors
    /// Holds a short description of what went wrong
    UrlParsing(String),

    /// Used for missing ids that were expected because of specific segments
    /// Holds the id type that was missing and the segments in the url that define the id type that
    /// should be extracted
    UrlSegmentExtraction(&'static str, &'static str),

    /// Used for invalid urls or missing elements in the url
    /// Holds a small description
    InvalidUrl(String),

    /// Used when the url was completely empty and did not contain any valid id
    EmptyUrl,

    /// Used when the extraction of the media size from the url failed
    UrlSizeExtract,

    /// Used for any missing response data
    /// Holds a ResponseDataError each holding a variant for the correct client   
    ResponseData(ResponseDataError),

    ReqwestError(String),
    /// Used when all tries were used for bypassing the youtube captcha
    /// Holds the total amount of tries that were used for trying to bypass the captcha.
    CaptchaBypassFailed(u16),
    /// Used when the youtube api returned an error and / or the response was invalid or unexpected
    YoutubeAPIReturn,

    Tokio(String),
    Deserialize(String),

    /// Use this when the thumbnail was tried to be extracted in a media even though it does not
    /// contain a thumbnail
    /// Holds the name of the media for better debugging
    NoThumbnail(String),

    /// Used when the playlist at an index was empty and did not contain the whished media
    /// Holds the playlist length and the index that was used for getting the song
    MediaNotContained(u8, u8),

    /// Used when there was an error when merging two bundle medias
    /// Holds a short description of what went wrong
    BundleMerge(&'static str),

    NoLowerItagFound,
    NoMatchingStream,
    NoMatchingThumbnail,
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

            // Url related
            Self::UrlParsing(e) => write!(f, "Error while parsing the url: {e}"),
            Self::UrlSegmentExtraction(e, s) => write!(f, "Url with segments '{s}' did not contain the expected id: {e}"),
            Self::InvalidUrl(e) => write!(f, "Invalid url: {e}"),
            Self::EmptyUrl => write!(f, "The url did not contain any valid id meaning the id collection is empty"),
            Self::UrlSizeExtract => write!(f, "Failed to extract the size from the url."),

            // Response data related
            Self::ResponseData(e) => write!(f, "Missing response data: {}", e.to_string()),

            // Response / Request related
            Self::YoutubeAPIReturn => write!(f, "The Youtube API returned an unexpected or invalid response. This could be caused by invalid ids or other parameters."),

            Self::Tokio(e) => write!(f, "tokio error: {}", e),

            Self::ReqwestError(e) => write!(f, "Reqwest failed: {e}"),
            Self::CaptchaBypassFailed(e) => {
                write!(f, "The captcha bypass failed after {} tries.", e)
            }

            Self::Deserialize(e) => write!(f, "Could not deserialize the response. {e}"),

            Self::BundleMerge(e) => write!(f, "Error while merging media bundles: {}", e),
            Self::NoThumbnail(e) => write!(f, "Tried to get the thumbnail for media '{}' but did not find any. Download with thumbnail to actually be able to extract it", e),
            Self::MediaNotContained(l, i) => write!(f, "Playlist did not contain the song at the index. Playlist length: {} Index: {}", l, i),

            Self::NoLowerItagFound => write!(f, "Could not find any lower itag."),
            Self::NoMatchingStream => write!(f, "No matching stream found for this itag."),
            Self::NoMatchingThumbnail => write!(f, "No matching thumbnail found"),
        }
    }
}

impl Error for YtuwuError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
