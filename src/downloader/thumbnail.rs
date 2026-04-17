use std::{
    fs, 
    io::Write, 
    path::{Path, PathBuf}
};

use anyhow::Result;
use bytes::Bytes;

use crate::{player_model::video_details::ThumbnailResolution};

#[derive(Debug)]
pub struct Thumbnail {
    name: String,
    data: Bytes,
    size: ThumbnailResolution,
}

#[derive(Debug)]
pub struct PlaylistThumbnail {
    data: Vec<Thumbnail>,
    size: ThumbnailResolution,
}

impl Thumbnail {
    pub fn new(data: Bytes, size: ThumbnailResolution, name: &str) -> Self {
        Self { data, size, name: name.to_owned() }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let mut file_path = PathBuf::from(path);
        
        let file_name = format!("{}.png", &self.name); 
        file_path.push(file_name);

        let mut file = fs::File::create(file_path)?;
        file.write_all(&self.data)?;
        Ok(())
    }
}

impl PlaylistThumbnail {
    pub fn new(data: Vec<Thumbnail>, size: ThumbnailResolution) -> Self {
        Self { data, size }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        fs::create_dir(path)?; 
        for thumbnail in self.data.iter() {
            thumbnail.save(&path)?    
        }
        Ok(())
    }
}


