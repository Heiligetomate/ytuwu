use std::fmt::Debug;

use anyhow::{Result, anyhow};
use bytes::Bytes;
use crate::downloader::downloaded::{DownloadedDualStreamMedia, DownloadedMedia};
use crate::downloader::media_stream::{MediaStream};
use crate::downloader::thumbnail::Thumbnail;
use crate::downloader::util::*;
use crate::player_model::itag::{AudioItag, VideoItag};
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

const CHUNK_SIZE: u32 = 1024 * 1024; 

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
    
    pub async fn download_chunk(&self, from: u32, to: u32, url: &str) -> Result<Bytes> {
        let client = reqwest::Client::new();
        let chunk_url = format!("{}&range={}-{}", url, from, to);
        let chunk = client.get(&chunk_url).send().await?.bytes().await?;
        Ok(chunk)
    }
    
    fn get_best_stream<I: Itag + Copy>(&self, itag: &I) -> Result<&str> {
        let streams = self.get_streams()?;
        let mut current_itag = *itag;
        let mut url: Option<&str> = streams.get_url_by_itag(&current_itag);
        while url.is_none() {
            current_itag = current_itag.next_best()?;
            url = streams.get_url_by_itag(&current_itag);
        }
        Ok(url.ok_or(anyhow!("no matching itag"))?)
    }

    pub async fn download_media_stream<I: Itag + Copy>(&self, itag: I) -> Result<I::Stream> {
        let url = self.get_best_stream(&itag)?; 
        let size = extract_size(url)?;
        let mut downloaded_stream = itag.new_stream();
        let mut current_position: u32 = 0;
        
        while size > current_position {
            println!(
                "downloading chunk {} to {}",
                current_position,
                current_position + CHUNK_SIZE
            );
            let chunk = self.download_chunk(current_position, current_position + CHUNK_SIZE, url)
                .await?;
            downloaded_stream.push_data(chunk);
            current_position += CHUNK_SIZE + 1
        }
        Ok(downloaded_stream)
    }    
    
    pub async fn download_dual_stream(&self, video_itag: VideoItag, audio_itag: AudioItag, thumbnail_resolution: &ThumbnailResolution) -> Result<DownloadedDualStreamMedia> {
        let video_stream = self.download_media_stream(video_itag).await?;
        let audio_stream = self.download_media_stream(audio_itag).await?;
        let thumbnail = self.download_thumbnail(thumbnail_resolution).await?;
        Ok(
            DownloadedDualStreamMedia::new(
                audio_stream, 
                video_stream, 
                thumbnail, 
                &self.title, 
                &self.player_response.get_author().ok_or(anyhow!("no author found. fix this shit"))?,
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

    pub async fn download_full<I>(self, itag: I, thumbnail_resolution: &ThumbnailResolution) -> Result<DownloadedMedia<I::Stream>> 
    where 
        I: Itag + Copy + Debug, 
        I::Stream: Debug,    
    {
        
        let thumbnail = self.download_thumbnail(&thumbnail_resolution).await?;
        let media = self.download_media_stream(itag).await?;
        
        let downloaded_media = DownloadedMedia::new(
            media,
            &self.title,  
            thumbnail,
            self.player_response.get_author().ok_or(anyhow!("no author found. fix this shit"))?,
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

