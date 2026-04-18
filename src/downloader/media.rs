use anyhow::{Result, anyhow};
use bytes::{BufMut, Bytes, BytesMut};
use crate::downloader::full::DownloadedMedia;
use crate::downloader::media_stream::MediaStream;
use crate::downloader::thumbnail::Thumbnail;
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
    pub title: String,  
    player_response: PlayerResponse,
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

    pub async fn download_media_stream(&self, itag: &Itag, chunk_count: u16) -> Result<MediaStream> {
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
        Ok(
            MediaStream::new(
                downloaded_stream.into(),
                itag.clone(),
                &self.title,
            )
        )
    }    

    pub async fn download_thumbnail(&self, resolution: &ThumbnailResolution) -> Result<Thumbnail> {
        let url = self.get_thumbnail_url(&resolution)?;
        let client = reqwest::Client::new();
        let thumbnail = client. 
            get(url)
            .send()
            .await?
            .bytes()
            .await?;
        Ok(
            Thumbnail::new(thumbnail, resolution.clone(), &self.title)
        )
    }

    pub async fn download_full(self, itag: &Itag, chunk_count: u16, thumbnail_resolution: &ThumbnailResolution) -> Result<DownloadedMedia> {
        
        let thumbnail = self.download_thumbnail(&thumbnail_resolution).await?;
        let media = self.download_media_stream(&itag, chunk_count).await?;
        
        let downloaded_media = DownloadedMedia::new(
            &self.title, 
            media, 
            self.generate_file_name(&itag),
            thumbnail,
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

