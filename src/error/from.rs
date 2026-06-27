use crate::error::YtuwuError;

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

impl From<url::ParseError> for YtuwuError {
    fn from(value: url::ParseError) -> Self {
        Self::UrlParsing(value.to_string())
    }
}
