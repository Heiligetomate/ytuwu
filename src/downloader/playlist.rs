use std::{fs, path::{Path, PathBuf}};

use anyhow::{Result, anyhow};

use crate::{
    browse_model::browse_response::BrowseResponse, downloader::{
        full::{DownloadedMedia, DownloadedPlaylist}, media::{ 
            Media, 
            MediaBrowse
        }, thumbnail::{PlaylistThumbnail, Thumbnail} 
    }, id_resolver::{
        BrowseId, 
        Id, 
        VideoId
    }, 
    name_trimmer, 
    player_model::{
        itag::Itag,
        video_details::ThumbnailResolution,
    }, request::shared::captcha_bypass
};

#[derive(Debug)]
pub struct PlaylistBrowse {
    browse_id: BrowseId,
}

#[derive(Debug)]
pub struct PlaylistContentBrowse {
    title: String, 
    media: Vec<MediaBrowse>,
}

#[derive(Debug)]
pub struct Playlist {
    title: String,
    media: Vec<Media>,
}

impl PlaylistBrowse {
    pub fn new(id: BrowseId) -> Self {
        Self { 
            browse_id: id 
        }
    }
    pub async fn browse(self) -> Result<PlaylistContentBrowse> {
        let response: BrowseResponse = captcha_bypass(crate::request::shared::Endpoint::Browse(self.browse_id), 2).await?;
        let ids = response.get_ids()?;
        let title = response.get_album_title()?.to_owned();
        let trimmed_title = name_trimmer::trim(title, "-");
        let media: Vec<MediaBrowse> = ids.iter().map(
            |id| {
                let video_id = VideoId::new(*id);
                MediaBrowse::new(video_id)
            }
        ).collect();
        Ok(
            PlaylistContentBrowse { title: trimmed_title, media }
        )
    }
}

impl PlaylistContentBrowse {
    pub async fn browse(mut self) -> Result<Playlist> {
        let mut media_items: Vec<Media> = Vec::new();
        for item in self.media.drain(..) {
            media_items.push(item.browse().await?);
        }
        Ok( 
            Playlist {
                media: media_items,
                title: self.title
            }
        )
    }
}

impl Playlist {
    pub async fn download_full<I: Itag + Copy>(mut self, itag: I, thumbnail_resolution: ThumbnailResolution) -> Result<DownloadedPlaylist<I>> {
        let mut downloaded: Vec<DownloadedMedia<I>> = Vec::new(); 
        for item in self.media.drain(..) {
            let downloaded_media = item.download_full(itag, 3, &thumbnail_resolution).await?; 
            downloaded.push(downloaded_media);
        }
        Ok(DownloadedPlaylist::new(&self.title, downloaded))
    }
    
    pub async fn download_thumbnails(&self, thumbnail_resolution: ThumbnailResolution) -> Result<PlaylistThumbnail> {
        let mut thumbnails = Vec::new();
        for item in self.media.iter() {
            let downloaded_thumbnail = item.download_thumbnail(&thumbnail_resolution).await?;
            thumbnails.push(downloaded_thumbnail);
        }
        Ok(
            PlaylistThumbnail::new(thumbnails, thumbnail_resolution)
        )
    } 
}


