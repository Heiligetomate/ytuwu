use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};
use bytes::{BufMut, Bytes, BytesMut};
use crate::downloader::util::*;
use crate::{
    id_resolver::VideoId, 
    name_trimmer::trim, 
    player_model::{
        itag::Itag, 
        player_response::PlayerResponse, 
        streaming_data::StreamingData, 
        video_details::{
            ThumbnailResolution, 
            VideoDetails
        }
    }, 
    request::shared::captcha_bypass
};


#[derive(Debug)]
pub struct MediaBrowse {
    video_id: VideoId, 
}

#[derive(Debug)]
pub struct Media {
    title: String,  
    player_response: PlayerResponse,
}

#[derive(Debug)]
pub struct DownloadedMedia {
    pub title: String,
    pub file_name: Option<String>,
    pub album: Option<String>,
    pub artist: Option<String>,
    pub thumbnail: Option<Bytes>,
    pub stream: Bytes,
    // file type & itag 
}

impl Media {
    fn get_streams(&self) -> Result<&StreamingData> {
        Ok(
            self
            .player_response
            .streaming_data
            .as_ref()
            .ok_or(anyhow!("no streaming data found"))?
        )
    }

    fn get_video_details(&self) -> Result<&VideoDetails> {
        Ok(
            self
            .player_response
            .video_details
            .as_ref()
            .ok_or(anyhow!("no video details found"))?
        )
    }

    fn get_thumbnail_url(&self, resolution: &ThumbnailResolution) -> Result<&str> {
        let url = self
            .get_video_details()?
            .thumbnail
            .url_by_resolution(resolution)
            .ok_or(anyhow!("no thumbnail found"))?;
        Ok(url)
    }
    
    fn get_mime_type_by_itag(&self, itag: &Itag) -> Result<&str> {
        if let Some(stream) = self.get_streams()?.get_stream_by_itag(&itag) {
            return Ok(stream.get_mime_type())
        }
        Err(anyhow!("could not extract mime type"))
    }
    
    pub fn generate_file_name(&self, itag: &Itag) -> Option<String> {
        if let Some(mime_type) = self.get_mime_type_by_itag(&itag).ok() {
            let file_name = file_name(mime_type, &self.title);
            println!("generated file name: {}", &file_name);
            return Some(file_name);
        } 
        
        None
    }
    
    pub async fn download_chunk(&self, from: u64, to: u64, url: &str) -> Result<Bytes> {
        let client = reqwest::Client::new();
        let chunk_url = format!("{}&range={}-{}", url, from, to);
        let chunk = client.get(&chunk_url).send().await?.bytes().await?;
        Ok(chunk)
    }

    pub async fn chunked_download(&self, itag: &Itag, chunk_count: u16) -> Result<Bytes> {
        let url = self.get_streams()?.get_url_by_itag(&itag).ok_or(anyhow!("couldnt get url from itag"))?; 
        let size = extract_size(url)?;
        let mut downloaded_stream = BytesMut::new();
        let chunk_size = size / chunk_count as u64;
        let mut current_position: u64 = 0;
        while size > current_position {
            println!(
                "downloading chunk {} to {}",
                current_position,
                current_position + chunk_size
            );
            let chunk = self.download_chunk(current_position, current_position + chunk_size, url)
                .await?;
            downloaded_stream.put(chunk);
            current_position += chunk_size + 1
        }
        Ok(downloaded_stream.into())
    }    

    pub async fn download_thumbnail(&self, resolution: &ThumbnailResolution) -> Result<Bytes> {
        let url = self.get_thumbnail_url(resolution)?;
        let client = reqwest::Client::new();
        let thumbnail = client. 
            get(url)
            .send()
            .await?
            .bytes()
            .await?;
        Ok(thumbnail)
    }

    pub async fn full_download(self, itag: &Itag, chunk_count: u16, thumbnail_resolution: &Option<ThumbnailResolution>) -> Result<DownloadedMedia> {
        let mut thumbnail_stream: Option<Bytes> = None;
        if let Some(resolution) = thumbnail_resolution {
            let thumbnail_bytes = self.download_thumbnail(resolution).await?;
            thumbnail_stream = Some(thumbnail_bytes);
        }
        
        let media_stream = self.chunked_download(itag, chunk_count).await?;
        let downloaded_media = DownloadedMedia::new(
            &self.title, 
            media_stream, 
            self.generate_file_name(itag),
            thumbnail_stream,
            self.player_response.get_author(),
        );

        Ok(downloaded_media)
    } 
}


impl MediaBrowse {
    pub fn new(id: VideoId) -> Self {
        Self {
            video_id: id,
        }
    }

    pub async fn browse(self) -> Result<Media> {
        let response: PlayerResponse = captcha_bypass(crate::request::shared::Endpoint::Player(self.video_id), 2).await?;
        let title = response.get_title().to_owned();
        let trimmed_title = trim(title, "-");
        Ok(
            Media {
                title: trimmed_title, 
                player_response: response,
            }
        )
    }
}

impl DownloadedMedia {
    
    fn new(title: &str, stream: Bytes, file_name: Option<String>, thumbnail: Option<Bytes>, author: Option<&str>) -> Self {
        let author = 
        {
            if let Some(auth) = author {
                Some(auth.to_owned())
            } else {
                None
            }
        };
        Self { album: None, artist: author, thumbnail, stream, title: title.to_owned(), file_name }
    }

    #[allow(unused)]
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let mut full_path = PathBuf::from(path);
        if path.is_dir() {
            if let Some(name) = &self.file_name {
                full_path.push(name);
            } else {
                println!("no file name found.");
                full_path.push(&self.title);
            }
        } else {
            println!("using given path");
        }
        let mut file = File::create_new(full_path)?;
        file.write_all(&self.stream)?;
        Ok(())
    }

    #[allow(unused)]
    pub fn save_thumbnail(&self) -> Result<()> {
        let thumbnail_stream = self.thumbnail.as_ref().ok_or(anyhow!("no thumbnail stream found"))?;
        let mut file = File::create_new("thumbnail.jpg")?;
        file.write_all(&thumbnail_stream)?;
        Ok(())
    }
}
