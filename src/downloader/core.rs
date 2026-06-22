use std::{fmt::Debug, sync::Arc};

use crate::{
    downloader::{
        builders::EmptyBuilder,
        channel::{ChannelBrowse, DwnBundelChannel, DwnChannel},
        media::{DwnBundleMedia, DwnMedia, MediaBrowse, ThumbRes},
        playlist::{DwnBundleList, Dwnlist, PlaylistBrowse},
        progress::{DefaultProgressHandler, EmptyHandler, HandleProgress},
        store::DownloadedStore,
        streams::Thumbnail,
        task_handler::TaskHandler,
    },
    error::Result,
    id_resolver::{
        IdCollection,
        types::{BrowseId, VideoId},
    },
    itags::{AnyItag, Itag},
    types::{ChannelId, ShortId},
};
use reqwest::Client;
use tokio::sync::Mutex;
use uuid::Uuid;

/// This is a wrapper for shared visitor data
/// This is important because it can save many requests by using the same visitor data for all of
/// the media
/// It has to be changable because as soon as the request with the current visitor data failes, it
/// retries with the new visitor data and changes the visitor data of the downloader struct to the
/// newly updated visitor data
pub type SharedVd = Arc<Mutex<Option<String>>>;

/// This struct is the core struct of the entire library. It handles the shared visitor data which
/// is used for bypassing the captcha.
/// It handles the download tasks
/// It handles the storage of the already downloaded tasks
/// It holds a shared reqwest client to avoid creating new clients each time
/// It holds a progress handler that the user can implement to display or track the current status
/// of the downloading progress
///
/// This can be either used with the easy api by calling .from_url which builds a new EmptyBuilder
/// that can then be configured for downloading whatever the user want to download or this can be
/// used through the api that gives more options but that lets you creates mistakes more easily
/// The "harder" api can be used by creating a new downloader object and calling functions like
/// download media on it. these functions require more detailled parametrs such as specific itags
/// or thumbnail resolutions
#[derive(Debug)]
pub struct Downloader {
    pub visitor_data: SharedVd,
    pub progress_handler: Arc<dyn HandleProgress + Send + Sync>,
    pub client: Client,
    pub task_handler: Mutex<TaskHandler>,
    pub storage: Mutex<DownloadedStore>,
}

impl Downloader {
    /// This creates a new downloader with default values and the progress_handler that was passed
    /// as a parameter. To use a normal downloader with a default progress handler, use
    /// Downloader::default() or Downloader::testing() for a completely empty handler
    #[must_use]
    pub fn new(progress_handler: Arc<dyn HandleProgress>) -> Arc<Self> {
        Arc::new(Self {
            visitor_data: Arc::new(Mutex::new(None)),
            progress_handler,
            client: Client::new(),
            task_handler: Mutex::new(TaskHandler::default()),
            storage: Mutex::new(DownloadedStore::default()),
        })
    }

    // TODO: either dont use this at all or use it for all
    /// Works all tasks in the current task storage and put the result into the downloaded storage
    pub async fn work(self: &Arc<Self>) {
        let handler = std::mem::take(&mut *self.task_handler.lock().await);
        handler.work(Arc::clone(&self)).await;
    }

    /// Returns a new EmptyBuilder from an url
    /// Can be configured to download different kind of stuff like playlists, channels and single
    /// media.
    /// It can also be configured to either download a single stream or both audio and video.
    /// A thumbnail is optional but possible to download with this.  
    pub fn from_url(self: &Arc<Self>, url: &str) -> Result<EmptyBuilder> {
        Ok(EmptyBuilder::new(Arc::clone(&self), IdCollection::from_url(url)?))
    }

    /// Creates a new Downloader instance with a default, ok looking progress bar
    pub fn default() -> Arc<Self> {
        Self::new(Arc::new(DefaultProgressHandler::new()))
    }

    /// Creates a new Downloader instance with a handler that does nothing
    pub fn testing() -> Arc<Self> {
        Self::new(Arc::new(EmptyHandler {}))
    }

    /// Download a thumbnail of a media with the given thumbnail resolution
    /// Fails if something went wrong while downloading the thumbnail or browsing the media which can
    /// occur when the id is invalid or the thumbnail resolution was not found for the browsed media
    pub async fn download_media_thumb(self: Arc<Self>, video_id: VideoId, resolution: ThumbRes) -> Result<Thumbnail> {
        Ok(MediaBrowse::new(video_id, Uuid::new_v4())
            .browse(self)
            .await?
            .download_thumbnail(resolution)
            .await?)
    }

    /// Download a single media stream with the given itag and video id
    /// Fails if something went wrong while downloading the media stream or browsing the media which
    /// can occur when the video id is invalid or the itag was not found for the browsed data
    /// Returns the stream that the generic itag holds
    pub async fn download_media_stream<I: Itag + Copy>(self: Arc<Self>, video_id: VideoId, itag: I) -> Result<I::Stream> {
        Ok(MediaBrowse::new(video_id, Uuid::new_v4())
            .browse(self)
            .await?
            .download_stream(itag)
            .await?)
    }

    /// Download a full media with the given itag and thumbnail resolution
    /// Fails if something went wrong whild downloading or browsing the media which can occur when
    /// the video id is invalid or the itag was not found for the browsed data
    /// Returns a DwnMedia which holds the mediastream that the generic itag holds and the thumbnail
    /// if the thumbnail resolution was not None
    pub async fn download_media<I: Itag>(self: Arc<Self>, video_id: VideoId, itag: I, thumbnail_resolution: Option<ThumbRes>) -> Result<DwnMedia<I::Stream>>
where {
        Ok(MediaBrowse::new(video_id, Uuid::new_v4())
            .browse(self)
            .await?
            .download(itag, thumbnail_resolution)
            .await?)
    }

    /// Download a full bundle media with the given itags and thumbnail resolution
    /// Fails if something went wrong whild downloading or browsing the media which can occur when
    /// the video id is invalid or the itag was not found for the browsed data
    /// Returns a DwnBundleMedia which holds the mediastreams for the itags and the thumbnail
    /// if the thumbnail resolution was not None
    pub async fn download_media_bundle(self: Arc<Self>, video_id: VideoId, itags: &[AnyItag], thumbnail_resolution: Option<ThumbRes>) -> Result<DwnBundleMedia> {
        Ok(MediaBrowse::new(video_id, Uuid::new_v4())
            .browse(self)
            .await?
            .download_bundle(itags, thumbnail_resolution)
            .await?)
    }

    /// Download a fill album with the given itag and thumbnail resolution
    /// Fails if something went wrong while downloading or browsing the list which can occur when
    /// the browse id is invalid or the itag was not found for the browsed data
    /// Returns a DwnList which holds the mediastream that the generic itag holds and the thumbnail
    /// if the thumbnail resolution was not None
    pub async fn download_album<I>(self: Arc<Self>, browse_id: BrowseId, itag: I, thumbnail_resolution: Option<ThumbRes>) -> Result<Dwnlist<I::Stream>>
    where
        I: Itag + 'static,
    {
        Ok(PlaylistBrowse::new(browse_id, self, Uuid::new_v4())
            .browse()
            .await?
            .browse()
            .await?
            .download(itag, thumbnail_resolution)
            .await?)
    }

    /// Download a full bundle list with the given itags and thumbnail resolution
    /// Fails if something went wrong whild downloading or browsing the list which can occur when
    /// the browse id is invalid or the itag was not found for the browsed data
    /// Returns a DwnBundleList which holds the mediastreams for the itags and the thumbnail
    /// if the thumbnail resolution was not None
    pub async fn download_bundle_album(self: Arc<Self>, browse_id: BrowseId, itags: &[AnyItag], thumbnail_resolution: Option<ThumbRes>) -> Result<DwnBundleList> {
        Ok(PlaylistBrowse::new(browse_id, self, Uuid::new_v4())
            .browse()
            .await?
            .browse()
            .await?
            .download_bundle(itags, thumbnail_resolution)
            .await?)
    }

    /// Download a full short with the given itags and thumbnail resolution
    /// Fails if something went wrong whild downloading or browsing the short which can occur when
    /// the short id is invalid or the itag was not found for the browsed data
    /// Returns a DwnMedia which holds the mediastream for the itag and the thumbnail
    /// if the thumbnail resolution was not None
    pub async fn download_short<I: Itag>(self: Arc<Self>, short_id: ShortId, itag: I, thumbnail_resolution: Option<ThumbRes>) -> Result<DwnMedia<I::Stream>>
where {
        let video_id = short_id.transform()?;

        Ok(self
            .download_media(video_id, itag, thumbnail_resolution)
            .await?)
    }

    /// Download a full channel with the given itag and thumbnail resolution
    /// Fails if something went wrong whild downloading or browsing the channel which can occur when
    /// the channel id is invalid or the itag was not found for the browsed data
    /// Returns a DwnChannel which holds the mediastream for the itag and the thumbnail
    /// if the thumbnail resolution was not None
    pub async fn download_channel<I>(self: Arc<Self>, channel_id: ChannelId, itag: I) -> Result<DwnChannel<I::Stream>>
    where
        I: Itag + 'static,
    {
        Ok(ChannelBrowse::new(channel_id, self, None)
            .await?
            .browse()
            .await?
            .browse()
            .await?
            .download(itag)
            .await?)
    }

    /// Download a full bundle channel with the given itags and thumbnail resolution
    /// Fails if something went wrong whild downloading or browsing the channel which can occur when
    /// the channel id is invalid or the itags were not found for the browsed data
    /// Returns a DwnBundelChannel which holds the mediastreams for the itags and the thumbnail
    /// if the thumbnail resolution was not None
    pub async fn download_channel_bundle(self: Arc<Self>, channel_id: ChannelId, itags: &'static [AnyItag]) -> Result<DwnBundelChannel> {
        Ok(ChannelBrowse::new(channel_id, self, None)
            .await?
            .browse()
            .await?
            .browse()
            .await?
            .download_bundle(itags)
            .await?)
    }
}
