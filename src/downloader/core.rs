use std::{fmt::Debug, sync::Arc};

use crate::{
    DwnBundleList, DwnBundleMedia, DwnMedia, Dwnlist, HandleProgress,
    downloader::{
        channel::{browse::ChannelBrowse, downloaded::DwnChannel},
        media::{browse::MediaBrowse, extracted_streams::ThumbRes},
        playlist::browse::PlaylistBrowse,
        progress::{DefaultProgressHandler, EmptyHandler},
    },
    error::Result,
    id_resolver::{browse_id::BrowseId, types::VideoId},
    itags::{AnyItag, Itag},
    streams::{MediaStream, Thumbnail},
    types::{ChannelId, ShortId},
};
use reqwest::Client;
use tokio::sync::Mutex;

pub type SharedVd = Arc<Mutex<Option<String>>>;

#[derive(Debug)]
pub struct Downloader {
    pub visitor_data: SharedVd,
    pub progress_handler: Arc<dyn HandleProgress + Send + Sync>,
    pub client: Client,
}

impl Downloader {
    #[must_use]
    pub fn new(progress_handler: Arc<dyn HandleProgress>) -> Arc<Self> {
        Arc::new(Self {
            visitor_data: Arc::new(Mutex::new(None)),
            progress_handler,
            client: Client::new(),
        })
    }

    pub fn default() -> Arc<Self> {
        Self::new(Arc::new(DefaultProgressHandler::new()))
    }

    pub fn testing() -> Arc<Self> {
        Self::new(Arc::new(EmptyHandler {}))
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

    pub async fn download_album<I>(self: Arc<Self>, browse_id: BrowseId, itag: I, thumbnail_resolution: Option<ThumbRes>) -> Result<Dwnlist<I::Stream>>
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

    pub async fn download_bundle_album(self: Arc<Self>, browse_id: BrowseId, itags: Vec<AnyItag>, thumbnail_resolution: Option<ThumbRes>) -> Result<DwnBundleList> {
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

    pub async fn download_channel<I>(self: Arc<Self>, channel_id: ChannelId, itag: I) -> Result<DwnChannel<I::Stream>>
    where
        I: Itag + Copy + Debug + Send + 'static,
        I::Stream: MediaStream + Debug + Send,
    {
        Ok(ChannelBrowse::new(channel_id, self)
            .await?
            .browse()
            .await?
            .download(itag)
            .await?)
    }

    // pub async fn download_playlist<I>(self: Arc<Self>, playlist_id: PlaylistId, itag: I) -> Result<Dwnlist<I::Stream>>
    // where
    //     I: Itag + Copy + Debug + Send + 'static,
    //     I::Stream: MediaStream + Debug + Send,
    // {
    //     Ok(PlaylistBrowse::new(playlist_id, self)
    //         .browse()
    //         .await?
    //         .browse()
    //         .await?
    //         .download(itag, None)
    //         .await?)
    // }
}
