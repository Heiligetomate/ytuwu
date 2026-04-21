use std::{fmt::Debug, fs::create_dir_all, path::{Path, PathBuf}};

use anyhow::{Result, anyhow};

use crate::{browse_model::full_response, downloader::{media_stream::MediaStream, metadata::PlaylistMetadata, playlist::Playlist, thumbnail::Thumbnail}, player_model::itag::Itag};

#[derive(Debug)]
pub struct DownloadedMedia<M: MediaStream + Debug> {
    // TODO: the file name thing is weird i think 
    pub title: String,
    pub file_name: Option<String>,
    pub artist: Option<String>,
    pub thumbnail: Thumbnail,
    pub stream: M,
}

impl<M: MediaStream + Debug> DownloadedMedia<M> {
    
    pub fn new(title: &str, stream: M, file_name: Option<String>, thumbnail: Thumbnail, author: Option<&str>) -> Self {
        Self { artist: author.map(|s| s.to_owned()), thumbnail, stream, title: title.to_owned(), file_name }
    }
    
    pub fn save_thumbnail(&self, path: &Path) -> Result<()> {
        self.thumbnail.save(path)?;
        Ok(())
    }

    pub fn save_media_stream(&self, path: &Path) -> Result<()> {
        self.stream.save(path, &self.title)?;
        Ok(())
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        self.save_thumbnail(&path)?;
        self.save_media_stream(&path)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct DownloadedPlaylist<M: MediaStream + Debug> {
    pub media: Vec<DownloadedMedia<M>>,
    pub metadata: PlaylistMetadata,
}

impl<M: MediaStream + Debug> DownloadedPlaylist<M> {
    pub fn new(title: &str, media: Vec<DownloadedMedia<M>>) -> Self {
        let metadata = PlaylistMetadata::new(title, media.len() as u16);
        Self { media, metadata }
    }
    
    pub fn save(&self, path: &Path) -> Result<()> {
        let mut full_path = PathBuf::from(path);
        full_path.push(&self.metadata.title);
        create_dir_all(&full_path)?;
        for media in self.media.iter() {
            media.save(&full_path)?
        }
        Ok(())
    }
}

