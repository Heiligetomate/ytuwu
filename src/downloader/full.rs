use std::{fs::create_dir_all, path::{Path, PathBuf}};

use anyhow::{Result, anyhow};

use crate::{browse_model::full_response, downloader::{media_stream::MediaStream, thumbnail::Thumbnail}};

#[derive(Debug)]
pub struct DownloadedMedia {
    // TODO: the file name thing is weird i think 
    pub title: String,
    pub file_name: Option<String>,
    pub artist: Option<String>,
    pub thumbnail: Thumbnail,
    pub stream: MediaStream,
}

impl DownloadedMedia {
    
    pub fn new(title: &str, stream: MediaStream, file_name: Option<String>, thumbnail: Thumbnail, author: Option<&str>) -> Self {
        Self { artist: author.map(|s| s.to_owned()), thumbnail, stream, title: title.to_owned(), file_name }
    }
    
    pub fn save_thumbnail(&self, path: &Path) -> Result<()> {
        self.thumbnail.save(path)?;
        Ok(())
    }

    pub fn save_media_stream(&self, path: &Path) -> Result<()> {
        self.stream.save(path)?;
        Ok(())
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        self.save_thumbnail(&path)?;
        self.save_media_stream(&path)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct DownloadedPlaylist {
    pub title: Option<String>,
    pub media: Vec<DownloadedMedia>,
    pub artist: Option<String>,
}

impl DownloadedPlaylist {
    pub fn new(title: &str, media: Vec<DownloadedMedia>) -> Self {
        Self { artist: None, media, title: Some(title.to_owned()) }
    }
    
    pub fn save(&self, path: &Path) -> Result<()> {
        let mut full_path = PathBuf::from(path);
        full_path.push(self.title.as_ref().ok_or(anyhow!("no album title found"))?);
        create_dir_all(&full_path)?;
        for media in self.media.iter() {
            media.save(&full_path)?
        }
        Ok(())
    }
}

