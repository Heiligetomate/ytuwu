use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};

use crate::{
    downloader::media::Media, 
    player_model::itag::Itag
};

pub fn generate_download_path(album_name: &str, media: &Media, itag: &Itag) -> Result<PathBuf> {
    println!("album name: {}", &album_name);
    let file_name = media.generate_file_name(&itag)
        .ok_or(anyhow!("couldnt generate file name"))?;

    let raw_path = format!("{}/{}", album_name, file_name);
    let full_path = PathBuf::from(raw_path);
    Ok(full_path)
}

pub fn extract_size(url: &str) -> Result<u64> {
    let size: u64 = url
        .split("clen=")
        .nth(1)
        .ok_or(anyhow!("failed to get size from url"))?
        .split('&')
        .next()
        .ok_or(anyhow!("failed to get size from url"))?
        .parse()?;
    Ok(size)
}

pub fn file_name(mime_type: &str, title: &str) -> String {
    let file_ending = file_ending_from_mime_type(mime_type);
    format!("{title}.{file_ending}")
}

pub fn file_ending_from_mime_type(mime_type: &str) -> &str {
    let parts: Vec<&str> = mime_type.split('/').collect();
    let res: Vec<&str> = parts[1].split(';').collect(); 
    
    res[0]
}

