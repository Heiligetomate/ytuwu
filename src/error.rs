use std::{error::Error, fmt::Display};

pub type Result<T> = std::result::Result<T, YtuwuError>;

#[derive(Debug, Clone)]
pub enum YtuwuError {
    BrowseDataNotFound(&'static str),
    PlayerDataNotFound(&'static str),
    ReqwestError(String),
    CaptchaBypassFailed(u16),

    YoutubeAPIReturn,
    Deserialize,

    NoLowerItagFound,
    NoMatchingItag,

    UrlSizeExtract,

    CreateFile,
    CreateDir,
    WriteToFile,
    InvalidPath,
    NoIdFound,
}

impl Display for YtuwuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BrowseDataNotFound(e) => write!(f, "Could not get data from response: {}.", e),
            Self::PlayerDataNotFound(e) => write!(f, "Could not get data from player response: {}.", e),
            Self::ReqwestError(e) => write!(f, "Reqwest failed: {e}"),
            Self::CaptchaBypassFailed(e) => write!(f, "The captcha bypass failed after {} tries.", e),
            Self::YoutubeAPIReturn => write!(f, "Youtube API gave an unexpected reply."),
            Self::Deserialize => write!(f, "Could not deserialize the response."),
            Self::NoLowerItagFound => write!(f, "Could not find any lower itag."),
            Self::NoMatchingItag => write!(f, "No matching stream found for this itag."),
            Self::UrlSizeExtract => write!(f, "Failed to extract the size from the url."),
            Self::CreateFile => write!(f, "Failed to create the file."),
            Self::CreateDir => write!(f, "Failed to create the dir."),
            Self::WriteToFile => write!(f, "Failed to write bytes to the file."),
            Self::InvalidPath => write!(f, "Invalid path format."),
            &Self::NoIdFound => write!(f, "Could not get id from collection"),
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
    fn from(_: serde_json::Error) -> Self {
        Self::Deserialize
    }
}

impl From<reqwest::Error> for YtuwuError {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value.to_string())
    }
}
