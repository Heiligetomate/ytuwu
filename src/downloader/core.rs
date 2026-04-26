use std::fmt::Debug;

use crate::{
    downloader::{
        downloaded::{DownloadedDualStreamMedia, DownloadedMedia, DownloadedPlaylist},
        media::MediaBrowse,
        media_stream::ShortVideoStream,
        playlist::PlaylistBrowse,
        thumbnail::{PlaylistThumbnail, Thumbnail},
    },
    error::Result,
    id_resolver::{BrowseId, ShortId, VideoId},
    itag::ShortVideoItag,
    player_model::{
        itag::{AudioItag, Itag, VideoItag},
        video_details::ThumbnailResolution,
    },
};

#[derive(Debug)]
pub struct Downloader {}

impl Downloader {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    #[rustfmt::skip]
    pub async fn download_thumbnail_media(&self, video_id: VideoId, resolution: ThumbnailResolution) -> Result<Thumbnail> {
        Ok(MediaBrowse::new(video_id)
            .browse()
            .await?
            .download_thumbnail(&resolution)
            .await?)
    }

    #[rustfmt::skip]
    pub async fn download_media_stream<I: Itag + Copy>(&self, video_id: VideoId, itag: I) -> Result<I::Stream> {
        Ok(MediaBrowse::new(video_id)
            .browse()
            .await?
            .download_media_stream(itag)
            .await?)
    }

    #[rustfmt::skip]
    pub async fn download_full_media<I>(&self, video_id: VideoId, itag: I, thumbnail_resolution: ThumbnailResolution) -> Result<DownloadedMedia<I::Stream>>
    where
        I: Itag + Copy + Debug,
        I::Stream: Debug,
    {
        Ok(MediaBrowse::new(video_id)
            .browse()
            .await?
            .download_full(itag, &thumbnail_resolution)
            .await?)
    }

    #[rustfmt::skip]
    pub async fn download_dual_media_stream(&self, video_id: VideoId, video_itag: VideoItag, audio_itag: AudioItag, thumbnail_resolution: ThumbnailResolution) -> Result<DownloadedDualStreamMedia> {
        Ok(MediaBrowse::new(video_id)
            .browse()
            .await?
            .download_dual_stream(video_itag, audio_itag, &thumbnail_resolution)
            .await?)
    }

    #[rustfmt::skip]
    pub async fn download_playlist_thumbnails(&self, browse_id: BrowseId, thumbnail_resolution: ThumbnailResolution) -> Result<PlaylistThumbnail> {
        Ok(PlaylistBrowse::new(browse_id)
            .browse()
            .await?
            .browse()
            .await?
            .download_thumbnails(thumbnail_resolution)
            .await?)
    }

    #[rustfmt::skip]
    pub async fn download_full_playlist<I>(&self, browse_id: BrowseId, itag: I, thumbnail_resolution: ThumbnailResolution) -> Result<DownloadedPlaylist<I::Stream>>
    where
        I: Itag + Copy + Debug,
        I::Stream: Debug,
    {
        Ok(PlaylistBrowse::new(browse_id)
            .browse()
            .await?
            .browse()
            .await?
            .download_full(itag, thumbnail_resolution)
            .await?)
    }

    #[rustfmt::skip]
    pub async fn download_short(&self, short_id: ShortId, video_itag: ShortVideoItag, audio_itag: AudioItag, thumbnail_resolution: ThumbnailResolution) -> Result<DownloadedPlaylist<ShortVideoStream>> {
        let id = MediaBrowse::from_short(short_id);
        Ok(
            id.browse()
            .await?
            .download_dual_stream(video_itag, audio_itag, &thumbnail_resolution)
            .await?
        )
    }
}
