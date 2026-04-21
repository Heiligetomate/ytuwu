use std::{fs, io::Write, path::{Path, PathBuf}};

use anyhow::{Result, anyhow};
use bytes::{BufMut, Bytes, BytesMut};

use crate::player_model::itag::{AudioItag, Itag, MuxedItag, ShortVideoItag, VideoItag};

pub trait MediaStream {
    //fn new<I: Itag>(itag: I) -> Self;
    fn save(&self, path: &Path, file_name: &str) -> Result<()>;
    fn get_itag(&self) -> impl Itag;
    fn get_data(&self) -> &BytesMut;
    fn push_data(&mut self, data: Bytes);
}

pub enum StreamWrapper {
    AudioStream(AudioStream),
    VideoStream(VideoStream),
    ShortVideoStream(ShortVideoStream),
    MuxedStream(MuxedStream),
}

#[derive(Debug)]
pub struct AudioStream {
    data: BytesMut,
    itag: AudioItag,
}

#[derive(Debug)]
pub struct VideoStream {
    data: BytesMut, 
    itag: VideoItag,
}

#[derive(Debug)]
pub struct ShortVideoStream {
    data: BytesMut,
    itag: ShortVideoItag,
}

#[derive(Debug)]
pub struct MuxedStream {
    data: BytesMut,
    itag: MuxedItag,
}

pub struct PlaylistMediaStream<I: Itag, M: MediaStream> {
    data: Vec<M>, 
    itag: I
}

impl MediaStream for AudioStream {

    fn get_data(&self) -> &BytesMut {
        &self.data
    }

    fn save(&self, path: &Path, file_name: &str) -> Result<()> {
        save_media_stream(path, &file_name, self)?;
        Ok(())
    }

    fn get_itag(&self) -> impl Itag {
        self.itag
    }

    fn push_data(&mut self, data: Bytes) {
        self.data.put(data);
    }
}

impl MediaStream for VideoStream {

    fn get_data(&self) -> &BytesMut {
        &self.data
    } 

    fn save(&self, path: &Path, file_name: &str) -> Result<()> {
        save_media_stream(path, &file_name, self)?;
        Ok(())
    }

    fn get_itag(&self) -> impl Itag {
        self.itag
    }

    fn push_data(&mut self, data: Bytes) {
        self.data.put(data);
    }
}

impl MediaStream for ShortVideoStream {

    fn get_data(&self) -> &BytesMut {
        &self.data
    } 

    fn save(&self, path: &Path, file_name: &str) -> Result<()> {
        save_media_stream(path, &file_name, self)?;
        Ok(())
    }

    fn get_itag(&self) -> impl Itag {
        self.itag
    }

    fn push_data(&mut self, data: Bytes) {
        self.data.put(data);
    }
}

impl MediaStream for MuxedStream {

    fn get_data(&self) -> &BytesMut {
        &self.data
    } 

    fn save(&self, path: &Path, file_name: &str) -> Result<()> {
        save_media_stream(path, &file_name, self)?;
        Ok(())
    }

    fn get_itag(&self) -> impl Itag {
        self.itag
    }

    fn push_data(&mut self, data: Bytes) {
        self.data.put(data);
    }
}

impl AudioStream {
    pub fn new(itag: AudioItag) -> Self {
        Self {
            data: BytesMut::new(), 
            itag
        }
    }
}

impl VideoStream {
    pub fn new(itag: VideoItag) -> Self {
        Self {
            data: BytesMut::new(), 
            itag
        }
    }
}

impl ShortVideoStream {
    pub fn new(itag: ShortVideoItag) -> Self {
        Self { 
            data: BytesMut::new(), 
            itag 
        }
    }
}

impl MuxedStream {
    pub fn new(itag: MuxedItag) -> Self {
        Self { 
            data: BytesMut::new(),
            itag,
        }
    }
}

fn save_media_stream(path: &Path, file_name: &str, media_stream: &impl MediaStream) -> Result<()> {
    let file_name = format!("{}.{}", file_name, media_stream.get_itag().get_mime_type());
    if !path.is_dir() { 
        return Err(anyhow!("expected a dir"))
    }
    let mut file_path = PathBuf::from(path);
    file_path.push(file_name);

    let mut file = fs::File::create(file_path)?;
    file.write_all(&media_stream.get_data())?;
    Ok(())
}

// impl<I, M> PlaylistMediaStream<I, M> 
// where I: Itag,
//       M: MediaStream 
// {
//     pub fn new(data: Vec<M>, itag: I) -> Self {
//         Self { data, itag }
//     }
// 
//     pub fn save(&self, path: &Path) -> Result<()> {
//         fs::create_dir_all(path)?; 
//         for stream in self.data.iter() {
//             stream.save(&path)?    
//         }
//         Ok(())
//     }
//  }

// impl<I> MediaStream<I> where I: Itag {
//     pub fn new(data: Bytes, itag: I, name: &str) -> Self {
//         Self { data, itag, name: name.to_owned() }
//     }
//     
//     /// the path is the directory where the file should be stored.  
//     pub fn save(&self, path: &Path) -> Result<()> {
//         if !path.is_dir() { 
//             return Err(anyhow!("expected a dir"))
//         }
//         let mut file_path = PathBuf::from(path);
//         let file_name = format!("{}.{}", &self.name, &self.itag.get_mime_type());
//         println!("generated file name: {}", &file_name);
//         file_path.push(file_name);
// 
//         let mut file = fs::File::create(file_path)?;
//         file.write_all(&self.data)?;
//         Ok(())
//     }
// 
//     pub fn get_data(self) -> Bytes {
//         self.data
//     }
// }




