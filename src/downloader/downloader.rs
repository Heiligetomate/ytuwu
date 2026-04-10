use anyhow::{Ok, Result};

use crate::{
    downloader::{
        media::{
            DownloadedMedia,
            MediaBrowse,
            Media,
        },
        playlist::{
            DownloadedPlaylist,
            PlaylistBrowse,
            Playlist,
            PlaylistContentBrowse,
        }
    },
    id_resolver::{
        BrowseId,  
        VideoId,
    }, player_model::{
        video_details::ThumbnailResolution,
        itag::Itag, 

    }, 
};

#[derive(Debug)]
pub struct Downloader {
    media_download_queue         : Vec<Media>,
    media_browse_queue           : Vec<MediaBrowse>,
    playlist_download_queue      : Vec<Playlist>,
    playlist_content_browse_queue: Vec<PlaylistContentBrowse>,
    playlist_browse_queue        : Vec<PlaylistBrowse>,
}

impl Downloader {
    pub fn new() -> Self {
        Self {
            media_download_queue         : Vec::new(),
            media_browse_queue           : Vec::new(),
            playlist_download_queue      : Vec::new(),
            playlist_content_browse_queue: Vec::new(),
            playlist_browse_queue        : Vec::new(),
        }
    }
   
    #[allow(unused)]
    pub fn add_playlist_browse(&mut self, browse_id: BrowseId) {
        let playlist_browse = PlaylistBrowse::new(browse_id);
        self.playlist_browse_queue.push(playlist_browse);
    }
    
    #[allow(unused)]
    pub fn add_media_browse(&mut self, video_id: VideoId) {
        let media_browse = MediaBrowse::new(video_id);
        self.media_browse_queue.push(media_browse); 
    }
    
    pub async fn browse_playlists(&mut self) -> Result<()> {
        for playlist in self.playlist_browse_queue.drain(..) {
            self.playlist_content_browse_queue.push(playlist.browse().await?);
        } 
        Ok(())
    }

    pub async fn content_browse_playlist(&mut self) -> Result<()> {
        for playlist in self.playlist_content_browse_queue.drain(..) {
            self.playlist_download_queue.push(playlist.browse().await?);
        }
        Ok(())
    }
    
    pub async fn browse_all(&mut self) -> Result<()> {
        self.browse_playlists().await?;
        self.content_browse_playlist().await?;
        for item in self.media_browse_queue.drain(..) {
            let media = item.browse().await?;
            self.media_download_queue.push(media);
        }
        Ok(())
    }
    
    #[allow(unused)]
    pub async fn playlist_download(&mut self, itag: &Itag, thumbnail_resolution: &Option<ThumbnailResolution>) -> Result<Vec<DownloadedPlaylist>> {
        let mut downloaded: Vec<DownloadedPlaylist> = Vec::new();
        for playlist in self.playlist_download_queue.drain(..) {
            let downloaded_playlist = playlist.download(itag, thumbnail_resolution).await?;
            downloaded.push(downloaded_playlist);
        }
        Ok(downloaded)
    }

    #[allow(unused)]
    pub async fn download_media(&mut self, itag: &Itag, thumbnail_resolution: &Option<ThumbnailResolution>) -> Result<Vec<DownloadedMedia>> {
        let mut downloaded: Vec<DownloadedMedia> = Vec::new();
        for item in self.media_download_queue.drain(..) {
            let downloaded_item = item.full_download(itag, 3, thumbnail_resolution).await?; 
            downloaded.push(downloaded_item);
        }  
        Ok(downloaded)
    }    
}


