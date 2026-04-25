use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use crate::error::{Result, YtuwuError};
use bytes::{BufMut, Bytes, BytesMut};

use crate::player_model::itag::{AudioItag, Itag, MuxedItag, ShortVideoItag, VideoItag};

pub trait MediaStream {
    fn save(&self, path: &Path, file_name: &str) -> Result<()>;
    fn get_itag(&self) -> impl Itag;
    fn get_data(&self) -> &BytesMut;
    fn push_data(&mut self, data: Bytes);
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

#[derive(Debug)]
pub struct PlaylistMediaStream<M: MediaStream> {
    pub data: Vec<M>,
}

impl<M: MediaStream> PlaylistMediaStream<M> {
    pub fn new(data: Vec<M>) -> Self {
        Self { data }
    }
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
        Self { data: BytesMut::new(), itag }
    }
}

impl VideoStream {
    pub fn new(itag: VideoItag) -> Self {
        Self { data: BytesMut::new(), itag }
    }
}

impl ShortVideoStream {
    pub fn new(itag: ShortVideoItag) -> Self {
        Self { data: BytesMut::new(), itag }
    }
}

impl MuxedStream {
    pub fn new(itag: MuxedItag) -> Self {
        Self { data: BytesMut::new(), itag }
    }
}

fn save_media_stream(path: &Path, file_name: &str, media_stream: &impl MediaStream) -> Result<()> {
    let file_name = format!("{}.{}", file_name, media_stream.get_itag().get_mime_type());
    if !path.is_dir() {
        return Err(YtuwuError::InvalidPath);
    }
    let mut file_path = PathBuf::from(path);
    file_path.push(file_name);

    let mut file = fs::File::create(file_path).map_err(|_| YtuwuError::CreateFile)?;
    file.write_all(&media_stream.get_data()).map_err(|_| YtuwuError::WriteToFile)?;
    Ok(())
}
