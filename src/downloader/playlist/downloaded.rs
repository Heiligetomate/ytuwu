use std::{
    fmt::Debug,
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use crate::{
    Result,
    downloader::{
        media::{DwnBundleMedia, DwnMedia},
        metadata::PlaylistMetadata,
        streams::MediaStream,
    },
    error::YtuwuError,
};

/// This struct represents a downloaded list with just one stream per media which contains
/// some metadata and a vec of DwnMedia which are all from the same MediaStream type.  
#[derive(Debug)]
pub struct DwnList<M: MediaStream + Debug> {
    pub media: Vec<DwnMedia<M>>,
    pub metadata: PlaylistMetadata,
}

/// This struct represents a downloaded list with multiple, unlimiterd streams per media which
/// contains some metadata adn a vec of DwnBundleMedia where every media sould have the same streams
#[derive(Debug)]
pub struct DwnBundleList {
    pub media: Vec<DwnBundleMedia>,
    pub metadata: PlaylistMetadata,
}

impl DwnBundleList {
    /// takes the downloaded media bundles and a title and returns a new DwnBundleList  
    /// Creates new metadata from the title and the len of the given media
    pub fn new(media: Vec<DwnBundleMedia>, title: &str) -> Self {
        let metadata = PlaylistMetadata::new(title, media.len() as u16);
        Self { media, metadata }
    }

    // TODO: This fails if there is no thumbnail even tho this function jsut says save
    /// Goes over every downloaded media bundle contained in self.media and saves that media with
    /// the given path
    /// Fails if any of the mediasave failed
    pub fn save(&self, path: &Path) -> Result<()> {
        for media in self.media.iter() {
            media.save_full(path)?;
        }
        Ok(())
    }

    /// Creates a new folder with the given path with the name of the playlist which is found in
    /// self.metadata.title
    /// Calles self.save after that with the new created folder path to save every media in that
    /// path
    /// Failes if either the folder was not created or any media failed to save
    pub fn save_with_dir(&self, path: &Path) -> Result<()> {
        let mut full_path = PathBuf::from(path);
        full_path.push(&self.metadata.title);
        create_dir_all(&full_path).map_err(|e| YtuwuError::CreateDir(Some(format!("Tried to save a playlist but the creation of the directory failed: {}", e.to_string()))))?;
        self.save(&full_path)?;

        Ok(())
    }
}

impl<M: MediaStream + Debug> DwnList<M> {
    /// takes the downloaded media bundles and a title and returns a new Dwnlist
    /// Creates new metadata from the title and the len of the given media  
    pub fn new(media: Vec<DwnMedia<M>>, title: &str) -> Self {
        let metadata = PlaylistMetadata::new(title, media.len() as u16);
        Self { media, metadata }
    }

    /// Goes over every media in self.media and calls save_media_stream on it which saves the
    /// mediastream to the given path
    /// Fails if any of the media stream failed to save
    pub fn save(&self, path: &Path) -> Result<()> {
        for media in self.media.iter() {
            media.save_media_stream(path)?
        }
        Ok(())
    }

    /// Creates a new directory at the given path with the name of the playlist which can be found
    /// under self.metadata.title
    /// After that, it calls self.save which saves every mediastream of the playlist to the given
    /// path which is the new created directory
    /// Failes if the creation of the directory failed or any of the media steams could not be saved
    pub fn save_with_dir(&self, path: &Path) -> Result<()> {
        let mut full_path = PathBuf::from(path);
        full_path.push(&self.metadata.title);

        create_dir_all(&full_path).map_err(|e| YtuwuError::CreateDir(Some(format!("Tried to save a playlist but the creation of the directory failed: {}", e.to_string()))))?;
        self.save(&full_path)?;

        Ok(())
    }
}
