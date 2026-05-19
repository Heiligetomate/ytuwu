use std::{
    fmt::Debug,
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use crate::{
    Result,
    downloader::{
        media::downloaded::{DownloadedMedia, RawDownloadedMedia},
        media_stream::MediaStream,
    },
    error::YtuwuError,
    metadata::PlaylistMetadata,
};

#[derive(Debug)]
pub struct DownloadedPlaylist<M: MediaStream + Debug> {
    pub media: Vec<DownloadedMedia<M>>,
    pub metadata: PlaylistMetadata,
}

pub struct RawDownloadedPlaylist<M: MediaStream + Debug> {
    pub media: Vec<RawDownloadedMedia<M>>,
}

impl<M: MediaStream + Debug> RawDownloadedPlaylist<M> {
    pub fn save(&self, path: &Path) -> Result<()> {
        for media in self.media.iter() {
            media.stream.save(path, &media.title)?;
        }
        Ok(())
    }

    pub fn new(media: Vec<RawDownloadedMedia<M>>) -> Self {
        Self { media }
    }
}

impl<M: MediaStream + Debug> DownloadedPlaylist<M> {
    pub fn new(title: &str, media: Vec<DownloadedMedia<M>>) -> Self {
        let metadata = PlaylistMetadata::new(title, media.len() as u16);
        Self { media, metadata }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let mut full_path = PathBuf::from(path);
        full_path.push(&self.metadata.title);
        create_dir_all(&full_path).map_err(|_| YtuwuError::CreateDir)?;
        for media in self.media.iter() {
            media.save(&full_path)?
        }
        Ok(())
    }
}
