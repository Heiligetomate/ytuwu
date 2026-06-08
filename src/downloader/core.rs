use std::{fmt::Debug, sync::Arc};

use crate::{
    DwnBundelChannel, DwnBundleList, DwnBundleMedia, DwnMedia, Dwnlist, IdCollection,
    downloader::{
        builders::empty::EmptyBuilder,
        channel::{browse::ChannelBrowse, downloaded::DwnChannel},
        media::{browse::MediaBrowse, extracted_streams::ThumbRes},
        playlist::browse::PlaylistBrowse,
        progress::EmptyHandler,
        store::DownloadedStore,
        task_handler::TaskHandler,
    },
    error::Result,
    id_resolver::types::{BrowseId, VideoId},
    itags::{AnyItag, Itag},
    progress::{DefaultProgressHandler, HandleProgress},
    streams::{AnyStream, MediaStream, Thumbnail},
    types::{ChannelId, ShortId},
};
use reqwest::Client;
use tokio::sync::Mutex;
use uuid::Uuid;

pub type SharedVd = Arc<Mutex<Option<String>>>;

#[derive(Debug)]
pub struct Downloader {
    pub visitor_data: SharedVd,
    pub progress_handler: Arc<dyn HandleProgress + Send + Sync>,
    pub client: Client,
    pub task_handler: Mutex<TaskHandler>,
    pub downloaded: Mutex<DownloadedStore>,
}

impl Downloader {
    #[must_use]
    pub fn new(progress_handler: Arc<dyn HandleProgress>) -> Arc<Self> {
        Arc::new(Self {
            visitor_data: Arc::new(Mutex::new(None)),
            progress_handler,
            client: Client::new(),
            task_handler: Mutex::new(TaskHandler::default()),
            downloaded: Mutex::new(DownloadedStore::default()),
        })
    }

    pub async fn work<I>(self: &Arc<Self>, itag: I) -> Result<()>
    where
        I: Itag + Copy + Debug + Send + 'static,
        I::Stream: MediaStream + Into<AnyStream> + 'static,
    {
        let handler = std::mem::take(&mut *self.task_handler.lock().await);
        let results = handler
            .work(Arc::clone(&self), itag)
            .await;

        self.downloaded
            .lock()
            .await
            .push_vec(results);

        Ok(())
    }

    pub fn from_url(self: &Arc<Self>, url: &str) -> Result<EmptyBuilder> {
        Ok(EmptyBuilder::new(Arc::clone(&self), IdCollection::from_url(url)?))
    }

    pub fn default() -> Arc<Self> {
        Self::new(Arc::new(DefaultProgressHandler::new()))
    }

    pub fn testing() -> Arc<Self> {
        Self::new(Arc::new(EmptyHandler {}))
    }

    pub async fn download_media_thumb(self: Arc<Self>, video_id: VideoId, resolution: ThumbRes) -> Result<Thumbnail> {
        Ok(MediaBrowse::new(video_id, Uuid::new_v4())
            .browse(self)
            .await?
            .download_thumbnail(resolution)
            .await?)
    }

    pub async fn download_media_stream<I: Itag + Copy>(self: Arc<Self>, video_id: VideoId, itag: I) -> Result<I::Stream> {
        Ok(MediaBrowse::new(video_id, Uuid::new_v4())
            .browse(self)
            .await?
            .download_stream(itag)
            .await?)
    }

    pub async fn download_media<I: Itag>(self: Arc<Self>, video_id: VideoId, itag: I, thumbnail_resolution: Option<ThumbRes>) -> Result<DwnMedia<I::Stream>>
where {
        Ok(MediaBrowse::new(video_id, Uuid::new_v4())
            .browse(self)
            .await?
            .download(itag, thumbnail_resolution)
            .await?)
    }

    pub async fn download_media_bundle(self: Arc<Self>, video_id: VideoId, itags: &[AnyItag], thumbnail_resolution: Option<ThumbRes>) -> Result<DwnBundleMedia> {
        Ok(MediaBrowse::new(video_id, Uuid::new_v4())
            .browse(self)
            .await?
            .download_bundle(itags, thumbnail_resolution)
            .await?)
    }

    pub async fn download_album<I>(self: Arc<Self>, browse_id: BrowseId, itag: I, thumbnail_resolution: Option<ThumbRes>) -> Result<Dwnlist<I::Stream>>
    where
        I: Itag + Copy + Debug + Send + 'static,
    {
        Ok(PlaylistBrowse::new(browse_id, self)
            .browse()
            .await?
            .browse()
            .await?
            .download(itag, thumbnail_resolution)
            .await?)
    }

    pub async fn download_bundle_album(self: Arc<Self>, browse_id: BrowseId, itags: &[AnyItag], thumbnail_resolution: Option<ThumbRes>) -> Result<DwnBundleList> {
        Ok(PlaylistBrowse::new(browse_id, self)
            .browse()
            .await?
            .browse()
            .await?
            .download_bundle(itags, thumbnail_resolution)
            .await?)
    }

    pub async fn download_short<I: Itag>(self: Arc<Self>, short_id: ShortId, itag: I, thumbnail_resolution: Option<ThumbRes>) -> Result<DwnMedia<I::Stream>>
where {
        let video_id = short_id.transform()?;

        Ok(self
            .download_media(video_id, itag, thumbnail_resolution)
            .await?)
    }

    pub async fn download_channel<I>(self: Arc<Self>, channel_id: ChannelId, itag: I) -> Result<DwnChannel<I::Stream>>
    where
        I: Itag + Copy + Debug + Send + 'static,
    {
        Ok(ChannelBrowse::new(channel_id, self)
            .await?
            .browse()
            .await?
            .download(itag)
            .await?)
    }

    pub async fn download_channel_bundle(self: Arc<Self>, channel_id: ChannelId, itags: &[AnyItag]) -> Result<DwnBundelChannel> {
        Ok(ChannelBrowse::new(channel_id, self)
            .await?
            .browse()
            .await?
            .download_bundle(itags)
            .await?)
    }
}
