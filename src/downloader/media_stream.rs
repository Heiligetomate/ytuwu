use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use crate::{
    downloader::mime_types::MimeType,
    error::{Result, YtuwuError},
    itag::LongVideoItag,
};
use bytes::{BufMut, Bytes, BytesMut};

use crate::models::itag::{AudioItag, Itag, MuxedItag, ShortVideoItag};

pub trait MediaStream {
    fn get_mime_type(&self) -> MimeType;
    fn save(&self, path: &Path, file_name: &str) -> Result<()>;
    fn get_data(&self) -> &BytesMut;
    fn push_data(&mut self, data: Bytes);
}

pub trait VideoStream: MediaStream {}

impl VideoStream for LongVideoStream {}

impl VideoStream for ShortVideoStream {}

#[derive(Debug)]
pub struct AudioStream {
    data: BytesMut,
    mime_type: MimeType,
}

#[derive(Debug)]
pub struct LongVideoStream {
    data: BytesMut,
    mime_type: MimeType,
}

#[derive(Debug)]
pub struct ShortVideoStream {
    data: BytesMut,
    mime_type: MimeType,
}

#[derive(Debug)]
pub struct MuxedStream {
    data: BytesMut,
    mime_type: MimeType,
}

impl MediaStream for AudioStream {
    fn get_data(&self) -> &BytesMut {
        &self.data
    }

    fn save(&self, path: &Path, file_name: &str) -> Result<()> {
        save_media_stream(path, &file_name, self)?;
        Ok(())
    }

    fn push_data(&mut self, data: Bytes) {
        self.data.put(data);
    }

    fn get_mime_type(&self) -> MimeType {
        self.mime_type
    }
}

impl MediaStream for LongVideoStream {
    fn get_data(&self) -> &BytesMut {
        &self.data
    }

    fn save(&self, path: &Path, file_name: &str) -> Result<()> {
        save_media_stream(path, &file_name, self)?;
        Ok(())
    }

    fn push_data(&mut self, data: Bytes) {
        self.data.put(data);
    }

    fn get_mime_type(&self) -> MimeType {
        self.mime_type
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

    fn push_data(&mut self, data: Bytes) {
        self.data.put(data);
    }

    fn get_mime_type(&self) -> MimeType {
        self.mime_type
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

    fn push_data(&mut self, data: Bytes) {
        self.data.put(data);
    }

    fn get_mime_type(&self) -> MimeType {
        self.mime_type
    }
}

impl AudioStream {
    pub fn new(itag: AudioItag) -> Self {
        Self {
            data: BytesMut::new(),
            mime_type: itag.get_mime_type(),
        }
    }
}

impl LongVideoStream {
    pub fn new(itag: LongVideoItag) -> Self {
        Self {
            data: BytesMut::new(),
            mime_type: itag.get_mime_type(),
        }
    }
}

impl ShortVideoStream {
    pub fn new(itag: ShortVideoItag) -> Self {
        Self {
            data: BytesMut::new(),
            mime_type: itag.get_mime_type(),
        }
    }
}

impl MuxedStream {
    pub fn new(itag: MuxedItag) -> Self {
        Self {
            data: BytesMut::new(),
            mime_type: itag.get_mime_type(),
        }
    }
}

fn save_media_stream<M>(path: &Path, file_name: &str, media_stream: &M) -> Result<()>
where
    M: MediaStream,
{
    let file_name = format!("{}.{}", file_name, media_stream.get_mime_type().as_str());
    if !path.is_dir() {
        return Err(YtuwuError::InvalidPath);
    }
    let mut file_path = PathBuf::from(path);
    file_path.push(file_name);

    let mut file = fs::File::create(file_path).map_err(|_| YtuwuError::CreateFile)?;
    file.write_all(&media_stream.get_data())
        .map_err(|_| YtuwuError::WriteToFile)?;
    Ok(())
}

#[derive(Debug)]
pub enum AnyStream {
    Audio(AudioStream),
    LongVideo(LongVideoStream),
    ShortVideo(ShortVideoStream),
    Muxed(MuxedStream),
}

impl AnyStream {
    pub fn into_dyn(&self) -> Box<&dyn MediaStream> {
        match self {
            Self::Audio(s) => Box::new(s),
            Self::LongVideo(s) => Box::new(s),
            Self::ShortVideo(s) => Box::new(s),
            Self::Muxed(s) => Box::new(s),
        }
    }
}
