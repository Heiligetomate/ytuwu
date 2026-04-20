use std::{fs, io::Write, path::{Path, PathBuf}};

use anyhow::{Result, anyhow};
use bytes::Bytes;

use crate::{downloader::metadata::MediaMetadata, player_model::itag::{AudioItag, Itag, VideoItag}};

pub trait MediaStream {
    fn save(&self, path: &Path, file_name: &str) -> Result<()>;
    fn get_data(self) -> Bytes;
    fn get_itag<I: Itag>(&self) -> I;
}

#[derive(Debug)]
pub struct AudioStream {
    data: Bytes,
    itag: AudioItag,
}

#[derive(Debug)]
pub struct VideoStream {
    data: Bytes, 
    itag: VideoItag
}

pub struct PlaylistMediaStream<I: Itag> {
    data: Vec<MediaStream<I>>, 
    itag: I
}

impl MediaStream for AudioStream {
    
    fn save(&self, path: &Path, file_name: &str) -> Result<()> {
        let file_name = format!("{}.{}", file_name, self.itag.get_mime_type());
        save_media_stream(path, &file_name, &self.data)?;
        Ok(())
    }

    fn get_data(self) -> Bytes {
        self.data
    }

    fn get_itag<I: Itag>(&self) -> I {
        self.itag
    }
}

impl MediaStream for VideoStream {

    fn save(&self, path: &Path, file_name: &str) -> Result<()> {
        let file_name = format!("{}.{}", file_name, self.itag.get_mime_type());
    }
}

impl AudioStream {
    fn new(data: Bytes, itag: AudioItag) -> Self {
        Self {
            data, 
            itag
        }
    }
}

impl VideoStream {
    fn new(data: Bytes, itag: VideoItag) -> Self {
        Self {
            data, 
            itag
        }
    }
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

fn save_media_stream<T: MediaStream>(path: &Path, file_name: &str, media_stream: &T) -> Result<()> {
    let file_name = format!("{}.{}", file_name, media_stream)
    if !path.is_dir() { 
        return Err(anyhow!("expected a dir"))
    }
    let mut file_path = PathBuf::from(path);
    file_path.push(file_name);

    let mut file = fs::File::create(file_path)?;
    file.write_all(data)?;
    Ok(())
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
