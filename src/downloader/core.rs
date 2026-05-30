use std::{fmt::Debug, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    DwnBundleList, DwnBundleMedia, DwnMedia, Dwnlist, HandleProgress,
    downloader::{
        channel::{browse::ChannelBrowse, downloaded::DwnChannel},
        media::{browse::MediaBrowse, extracted_streams::ThumbRes},
        playlist::browse::PlaylistBrowse,
    },
    error::Result,
    id_resolver::{
        id::MakeChannelId,
        types::{AlbumId, VideoId},
    },
    itags::{AnyItag, Itag},
    streams::{MediaStream, Thumbnail},
    types::{PlaylistId, ShortId},
};

pub type SharedVd = Arc<Mutex<Option<String>>>;

#[derive(Debug)]
pub struct Downloader {
    pub visitor_data: SharedVd,
    pub progress_handler: Arc<dyn HandleProgress + Send + Sync>,
}

impl Downloader {
    #[must_use]
    pub fn new(progress_handler: Arc<dyn HandleProgress>) -> Self {
        Self {
            visitor_data: Arc::new(Mutex::new(None)),
            progress_handler,
        }
    }

    pub async fn download_media_thumb(self: Arc<Self>, video_id: VideoId, resolution: ThumbRes) -> Result<Thumbnail> {
        Ok(MediaBrowse::new(video_id)
            .browse(self)
            .await?
            .download_thumbnail(resolution)
            .await?)
    }

    pub async fn download_media_stream<I: Itag + Copy>(self: Arc<Self>, video_id: VideoId, itag: I) -> Result<I::Stream> {
        Ok(MediaBrowse::new(video_id)
            .browse(self)
            .await?
            .download_stream(itag)
            .await?)
    }

    pub async fn download_media<I>(self: Arc<Self>, video_id: VideoId, itag: I, thumbnail_resolution: Option<ThumbRes>) -> Result<DwnMedia<I::Stream>>
    where
        I: Itag + Copy + Debug,
        I::Stream: Debug,
    {
        Ok(MediaBrowse::new(video_id)
            .browse(self)
            .await?
            .download(itag, thumbnail_resolution)
            .await?)
    }

    pub async fn download_media_bundle(self: Arc<Self>, video_id: VideoId, itags: Vec<AnyItag>, thumbnail_resolution: Option<ThumbRes>) -> Result<DwnBundleMedia> {
        Ok(MediaBrowse::new(video_id)
            .browse(self)
            .await?
            .download_streams(itags, thumbnail_resolution)
            .await?)
    }

    pub async fn download_album<I>(self: Arc<Self>, browse_id: AlbumId, itag: I, thumbnail_resolution: Option<ThumbRes>) -> Result<Dwnlist<I::Stream>>
    where
        I: Itag + Copy + Debug + Send + 'static,
        I::Stream: MediaStream + Debug + Send,
    {
        Ok(PlaylistBrowse::new(browse_id, self)
            .browse()
            .await?
            .browse()
            .await?
            .download(itag, thumbnail_resolution)
            .await?)
    }

    pub async fn download_bundle_album(self: Arc<Self>, browse_id: AlbumId, itags: Vec<AnyItag>, thumbnail_resolution: Option<ThumbRes>) -> Result<DwnBundleList> {
        Ok(PlaylistBrowse::new(browse_id, self)
            .browse()
            .await?
            .browse()
            .await?
            .download_streams(itags, thumbnail_resolution)
            .await?)
    }

    pub async fn download_short<I>(self: Arc<Self>, short_id: ShortId, itag: I, thumbnail_resolution: Option<ThumbRes>) -> Result<DwnMedia<I::Stream>>
    where
        I: Itag + Copy + Debug,
        I::Stream: Debug,
    {
        let video_id = short_id.transform()?;

        Ok(self
            .download_media(video_id, itag, thumbnail_resolution)
            .await?)
    }

    pub async fn download_channel<I, C>(self: Arc<Self>, channel_id: C, itag: I) -> Result<DwnChannel<I::Stream>>
    where
        I: Itag + Copy + Debug + Send + 'static,
        I::Stream: MediaStream + Debug + Send,
        C: MakeChannelId,
    {
        let id = channel_id.transform().await?;
        Ok(ChannelBrowse::new(id, self)
            .browse()
            .await?
            .download(itag)
            .await?)
    }

    pub async fn download_playlist<I>(self: Arc<Self>, playlist_id: PlaylistId, itag: I) -> Result<Dwnlist<I::Stream>>
    where
        I: Itag + Copy + Debug + Send + 'static,
        I::Stream: MediaStream + Debug + Send,
    {
        Ok(PlaylistBrowse::new(playlist_id, self)
            .browse()
            .await?
            .browse()
            .await?
            .download(itag, None)
            .await?)
    }
}
