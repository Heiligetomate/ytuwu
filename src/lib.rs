pub mod error;
pub mod id_resolver;

mod downloader;
mod models;
mod name_trimmer;
mod request;

#[cfg(test)]
mod tests;
pub use downloader::itags;
pub use downloader::progress;
// pub use downloader::media::extracted_streams::ThumbRes;
// pub use downloader::progress;
// pub use downloader::streams;
// pub use downloader::{channel::downloaded::*, core::Downloader, media::downloaded::*, metadata, playlist::downloaded::*};
pub use downloader::Downloader;
pub use error::Result;
pub use id_resolver::types;

// TODO: better pub use
