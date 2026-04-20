use anyhow::{Ok, Result};

use crate::{
    downloader::{
        full::{DownloadedMedia, DownloadedPlaylist},
        media::MediaBrowse, media_stream::MediaStream, playlist::PlaylistBrowse, thumbnail::{PlaylistThumbnail, Thumbnail}
    },
    id_resolver::{
        BrowseId, VideoId
    }, player_model::{
        itag::{Itag, VideoItag}, video_details::ThumbnailResolution 

    }, 
};

#[derive(Debug)]
pub struct Downloader {}

impl Downloader {

    #[must_use]
    pub fn new() -> Self {
        Self {  }
    }
    
    #[allow(unused)]
    pub async fn download_thumbnail_media(&self, video_id: VideoId, resolution: ThumbnailResolution) -> Result<Thumbnail> {
        Ok(
            MediaBrowse::new(video_id)
                .browse()
                .await?
                .download_thumbnail(&resolution)
                .await?
        )
    }

    #[allow(unused)]
    pub async fn download_media_stream<I: Itag + Copy>(&self, video_id: VideoId, itag: I) -> Result<MediaStream<I>> {
        Ok(
            MediaBrowse::new(video_id)
                .browse()
                .await?
                .download_media_stream(itag, 3)
                .await?
        )
    }
    
    #[allow(unused)]
    pub async fn download_full_media<I: Itag + Copy>(&self, video_id: VideoId, itag: I, thumbnail_resolution: ThumbnailResolution) -> Result<DownloadedMedia<I>> {
        Ok(
            MediaBrowse::new(video_id)
                .browse()
                .await?
                .download_full(itag, 3, &thumbnail_resolution)
                .await?
        )
    }

    #[allow(unused)]
    pub async fn download_playlist_thumbnails(&self, browse_id: BrowseId, thumbnail_resolution: ThumbnailResolution) -> Result<PlaylistThumbnail> {
        Ok(
            PlaylistBrowse::new(browse_id) 
                .browse()
                .await?
                .browse()
                .await?
                .download_thumbnails(thumbnail_resolution)
                .await?
        )
    }
    
    #[allow(unused)]
    pub async fn download_full_playlist<I: Itag + Copy>(&self, browse_id: BrowseId, itag: I, thumbnail_resolution: ThumbnailResolution) -> Result<DownloadedPlaylist<I>> {
        Ok(
            PlaylistBrowse::new(browse_id)
                .browse()
                .await?
                .browse()
                .await?
                .download_full(itag, thumbnail_resolution)
                .await?
        )
    }

    pub async fn download_full_video(&self, browse_id: BrowseId, itag: VideoItag, thumbnail_resolution: ThumbnailResolution) -> Result<> 
}


