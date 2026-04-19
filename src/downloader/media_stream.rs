use std::{fs, io::Write, path::{Path, PathBuf}};

use anyhow::{Result, anyhow};
use bytes::Bytes;

use crate::player_model::itag::Itag;

#[derive(Debug)]
pub struct MediaStream<I: Itag> {
    name: String,
    data: Bytes,
    itag: I,
}

pub struct PlaylistMediaStream<I: Itag> {
    data: Vec<MediaStream<I>>, 
    itag: I
}





impl<I> MediaStream<I> where I: Itag {
    pub fn new(data: Bytes, itag: I, name: &str) -> Self {
        Self { data, itag, name: name.to_owned() }
    }
    
    /// the path is the directory where the file should be stored.  
    pub fn save(&self, path: &Path) -> Result<()> {
        if !path.is_dir() { 
            return Err(anyhow!("expected a dir"))
        }
        let mut file_path = PathBuf::from(path);
        let file_name = format!("{}.{}", &self.name, &self.itag.get_mime_type());
        println!("generated file name: {}", &file_name);
        file_path.push(file_name);

        let mut file = fs::File::create(file_path)?;
        file.write_all(&self.data)?;
        Ok(())
    }

    pub fn get_data(self) -> Bytes {
        self.data
    }
}

impl<I> PlaylistMediaStream<I> where I: Itag {
    pub fn new(data: Vec<MediaStream<I>>, itag: I) -> Self {
        Self { data, itag }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path)?; 
        for stream in self.data.iter() {
            stream.save(&path)?    
        }
        Ok(())
    }
 }
