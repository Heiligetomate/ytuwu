use anyhow::{Ok, Result};

use crate::{
    downloader::{
        media::{
            DownloadedMedia, MediaBrowse
        }, media_stream::MediaStream, playlist::{
            DownloadedPlaylist,
            PlaylistBrowse,
        }, thumbnail::{PlaylistThumbnail, Thumbnail}
    },
    id_resolver::{
        BrowseId, VideoId
    }, player_model::{
        itag::Itag, video_details::ThumbnailResolution 

    }, 
};

#[derive(Debug)]
pub struct Downloader {
}

impl Downloader {
    pub fn new() -> Self {
        Self {  }
    }
    pub async fn download_thumbnail_media(&self, video_id: VideoId, resolution: ThumbnailResolution) -> Result<Thumbnail> {
        Ok(
            MediaBrowse::new(video_id)
                .browse()
                .await?
                .download_thumbnail(&resolution)
                .await?
        )
    }

    pub async fn download_media_stream(&self, video_id: VideoId, itag: &Itag) -> Result<MediaStream> {
        Ok(
            MediaBrowse::new(video_id)
                .browse()
                .await?
                .download_media_stream(itag, 3)
                .await?
        )
    }

    pub async fn download_full_media(&self, video_id: VideoId, itag: &Itag, thumbnail_resolution: ThumbnailResolution) -> Result<DownloadedMedia> {
        Ok(
            MediaBrowse::new(video_id)
                .browse()
                .await?
                .download_full(itag, 3, &thumbnail_resolution)
                .await?
        )
    }

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

    pub async fn download_full_playlist(&self, browse_id: BrowseId, itag: &Itag, thumbnail_resolution: ThumbnailResolution) -> Result<DownloadedPlaylist> {
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
}


