use std::{fs, io::Write, path::{Path, PathBuf}};

use anyhow::Result;
use bytes::Bytes;

use crate::player_model::itag::Itag;

#[derive(Debug)]
pub struct MediaStream {
    name: String,
    data: Bytes,
    itag: Itag, 
}

pub struct PlaylistMediaStream {
    data: Vec<MediaStream>, 
    itag: Itag
}

impl MediaStream {
    pub fn new(data: Bytes, itag: Itag, name: &str) -> Self {
        Self { data, itag, name: name.to_owned() }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let mut file_path = PathBuf::from(path);
        
        let file_name = format!("{}.png", &self.name); 
        file_path.push(file_name);

        let mut file = fs::File::create(file_path)?;
        file.write_all(&self.data)?;
        Ok(())
    }

    pub fn get_data(self) -> Bytes {
        self.data
    }
}

impl PlaylistMediaStream {
    pub fn new(data: Vec<MediaStream>, itag: Itag) -> Self {
        Self { data, itag }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        fs::create_dir(path)?; 
        for stream in self.data.iter() {
            stream.save(&path)?    
        }
        Ok(())
    }
 }
