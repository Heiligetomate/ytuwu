use std::{
    fmt::Debug,
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use crate::{
    DwnBundleMedia, Result,
    downloader::{media::downloaded::DwnMedia, media_stream::MediaStream},
    error::YtuwuError,
    metadata::PlaylistMetadata,
};

#[derive(Debug)]
pub struct Dwnlist<M: MediaStream + Debug> {
    pub media: Vec<DwnMedia<M>>,
    pub metadata: PlaylistMetadata,
}

#[derive(Debug)]
pub struct DwnBundleList {
    pub media: Vec<DwnBundleMedia>,
    pub metadata: PlaylistMetadata,
}

impl DwnBundleList {
    pub fn new(media: Vec<DwnBundleMedia>, title: &str) -> Self {
        let metadata = PlaylistMetadata::new(title, media.len() as u16);
        Self { media, metadata }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        for media in self.media.iter() {
            media.save_full(path)?;
        }
        Ok(())
    }

    pub fn save_with_dir(&self, path: &Path) -> Result<()> {
        let mut full_path = PathBuf::from(path);
        full_path.push(&self.metadata.title);
        create_dir_all(&full_path).map_err(|_| YtuwuError::CreateDir)?;
        self.save(&full_path)?;

        Ok(())
    }
}

impl<M: MediaStream + Debug> Dwnlist<M> {
    pub fn new(media: Vec<DwnMedia<M>>, title: &str) -> Self {
        let metadata = PlaylistMetadata::new(title, media.len() as u16);
        Self { media, metadata }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let mut full_path = PathBuf::from(path);
        full_path.push(&self.metadata.title);
        create_dir_all(&full_path).map_err(|_| YtuwuError::CreateDir)?;
        for media in self.media.iter() {
            media.save_media_stream(&full_path)?
        }
        Ok(())
    }

    pub fn save_with_dir(&self, path: &Path) -> Result<()> {
        let mut full_path = PathBuf::from(path);
        full_path.push(&self.metadata.title);
        create_dir_all(&full_path).map_err(|_| YtuwuError::CreateDir)?;
        self.save(&full_path)?;

        Ok(())
    }
}
