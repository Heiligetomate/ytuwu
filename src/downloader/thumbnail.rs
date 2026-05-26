use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use crate::{
    downloader::mime_types::MimeType,
    error::{Result, YtuwuError},
};
use bytes::Bytes;

#[derive(Debug)]
pub struct Thumbnail {
    name: String,
    data: Bytes,
}

impl Thumbnail {
    pub fn new(data: Bytes, name: &str) -> Self {
        Self { data, name: name.to_owned() }
    }

    pub fn bytes(&self) -> &Bytes {
        &self.data
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if !path.is_dir() {
            return Err(YtuwuError::InvalidPath);
        }
        let mut file_path = PathBuf::from(path);
        let file_name = format!("{}.{}", &self.name, MimeType::Png.as_str());
        file_path.push(file_name);

        let mut file = fs::File::create(path).map_err(|_| YtuwuError::CreateFile)?;
        file.write_all(&self.data)
            .map_err(|_| YtuwuError::WriteToFile)?;

        Ok(())
    }
}

// TODO: Maybe use this as a media stream
