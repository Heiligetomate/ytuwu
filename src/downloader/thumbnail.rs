use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use crate::error::{Result, YtuwuError};
use bytes::Bytes;

#[derive(Debug)]
pub struct Thumbnail {
    name: String,
    data: Bytes,
}

#[derive(Debug)]
pub struct PlaylistThumbnail {
    data: Vec<Thumbnail>,
}

impl Thumbnail {
    pub fn new(data: Bytes, name: &str) -> Self {
        Self {
            data,
            name: name.to_owned(),
        }
    }

    // dont use this lmao
    pub fn save_file(&self, path: &Path) -> Result<()> {
        let mut file = fs::File::create(path).map_err(|_| YtuwuError::CreateFile)?;
        file.write_all(&self.data)
            .map_err(|_| YtuwuError::WriteToFile)?;
        Ok(())
    }

    /// the path is the directory where the file should be stored.  
    pub fn save(&self, path: &Path) -> Result<()> {
        if !path.is_dir() {
            return Err(YtuwuError::InvalidPath);
        }
        let mut file_path = PathBuf::from(path);
        let file_name = format!("{}.png", &self.name);
        file_path.push(file_name);

        self.save_file(&file_path)?;
        Ok(())
    }
}

impl PlaylistThumbnail {
    pub fn new(data: Vec<Thumbnail>) -> Self {
        Self { data }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path).map_err(|_| YtuwuError::CreateDir)?;
        for thumbnail in self.data.iter() {
            thumbnail.save(&path)?
        }
        Ok(())
    }
}
