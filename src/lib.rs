pub mod error;
mod browse_model;
mod downloader;
mod player_model;
mod request;
pub mod id_resolver;
mod name_trimmer;
mod shared_traits;

#[cfg(test)]
mod test;

pub use player_model::{
    itag,
    video_details::ThumbnailResolution,
};

//pub use error::Result;

pub use downloader::{
    core::Downloader, 
    downloaded,
    metadata,
};

