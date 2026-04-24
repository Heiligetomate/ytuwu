use std::{error::Error, fmt::Display};

pub type Result<T> = std::result::Result<T, YtuwuError>;

#[derive(Debug)]
pub enum YtuwuError {
    GetResponseData(String),
    YoutubeAPIReturn,
    Deserialize,
}

impl Display for YtuwuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::YoutubeAPIReturn      => write!(f, "Youtube API gave an unexpected reply"),
            Self::Deserialize           => write!(f, "Could not deserialize the response"),
            Self::GetResponseData(e)    => write!(f, "Could not get data from response: {}", e),
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


