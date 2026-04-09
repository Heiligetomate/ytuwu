use std::{fs, path::{Path, PathBuf}};

use anyhow::{Result, anyhow};

use crate::{
    browse_model::{browse_response::BrowseResponse, playlist_renderer}, downloader::{
        media::{
            DownloadedMedia, 
            Media, 
            MediaBrowse
        }, 
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

#[derive(Debug)]
pub struct DownloadedPlaylist {
    pub title: Option<String>,
    pub media: Vec<DownloadedMedia>,
    pub artist: Option<String>,
    pub thumbnail: Option<String>,
}

impl DownloadedPlaylist {
    fn new(title: &str, media: Vec<DownloadedMedia>) -> Self {
        Self { artist: None, thumbnail: None, media, title: Some(title.to_owned()) }
    }
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
    pub async fn download(mut self, itag: &Itag, thumbnail_resolution: &Option<ThumbnailResolution>) -> Result<DownloadedPlaylist> {
        let mut downloaded: Vec<DownloadedMedia> = Vec::new();
        for item in self.media.drain(..) {
            let downloaded_media = item.full_download(itag, 3, &thumbnail_resolution).await?; 
            downloaded.push(downloaded_media);
        }
        Ok(DownloadedPlaylist::new(&self.title, downloaded))
    }
}

impl DownloadedPlaylist {
    // ich kotze im strahl
    pub fn save(&self, path: Option<&Path>) -> Result<()> {
        let mut playlist = PathBuf::new(); 
        if let Some(playlist_path) = path {
            fs::create_dir(playlist_path)?;
            playlist.push(playlist_path);
        } else {
            if let Some(title) = &self.title {
                fs::create_dir(title)?;
                playlist.push(title);
            } else {
                return Err(anyhow!("no valid dir or title"));
            }
        }

        for media in self.media.iter() {
            media.save_to_file(&playlist)?;
        } 
        Ok(())
    }
}
