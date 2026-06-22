use std::{fmt::Debug, sync::Arc};

use uuid::Uuid;

use crate::{
    Result,
    downloader::{
        Downloader,
        media::{DwnBundleMedia, DwnMedia, MediaBrowse},
        streams::AnyStream,
    },
    itags::{AnyItag, Itag},
    types::VideoId,
};

/// This struct represents a task that gets pushed onto the task handler
/// It holds all information to download it and to extract it out of the downloader storage
/// The itag is an any itag to prevent usign dynamic traits
/// Channel id shoud just be Some if its contained in a channel, playlist should just be some if the
/// song is contained in a playlist. The id is the equal to the one stored in the Media instance for
/// this task
#[derive(Debug)]
pub struct DownloadTask {
    itag: AnyItag,
    id: Uuid,
    video_id: VideoId,
    playlist_id: Option<Uuid>,
    channel_id: Option<Uuid>,
}

/// This struct represents a bundle task that gets pushed onto the task handler
/// It holds all information to download it and to extract it out of the downloader storage
/// The itags are a slice with AnyItags to prevent dynamic traits
/// Channel id shoud just be Some if its contained in a channel, playlist should just be some if the
/// song is contained in a playlist. The id is the equal to the one stored in the Media instance for
/// this task
#[derive(Debug)]
pub struct BundleDownloadTask {
    id: Uuid,
    video_id: VideoId,
    playlist_id: Option<Uuid>,
    channel_id: Option<Uuid>,
    itags: &'static [AnyItag],
}

/// This struct gets created when a DownloadTask finished. it now holds the downloaded media with
/// an AnyStream so that the task handler can hold multiple finished tasks of different types
/// The id, playlist_id and channel_id are the same of the DownloadTask and are important to
/// identify the downloaded media in the downloader storage later.
#[derive(Debug)]
pub struct FinishedTask {
    pub data: DwnMedia<AnyStream>,
    pub id: Uuid,
    pub playlist_id: Option<Uuid>,
    pub channel_id: Option<Uuid>,
}

/// This struct gets created when a DownloadBundleTask finished. it now holds the downloaded
/// bundle media holding every stream of the itag that was given in the download bundle task
/// The id, playlist_id and channel_id are the same of the DownloadTask and are important to
/// identify the downloaded bundle media in the downloader storage later.
#[derive(Debug)]
pub struct FinishedBundleTask {
    pub data: DwnBundleMedia,
    pub id: Uuid,
    pub playlist_id: Option<Uuid>,
    pub channel_id: Option<Uuid>,
}

impl FinishedTask {
    /// Creates a new FinishedTask with the given parameters
    pub fn new(media: DwnMedia<AnyStream>, id: Uuid, playlist_id: Option<Uuid>, channel_id: Option<Uuid>) -> Self {
        Self { data: media, id, playlist_id, channel_id }
    }
}

impl FinishedBundleTask {
    /// Creates a new FinishedBundleTask with the given paramets
    pub fn new(media: DwnBundleMedia, id: Uuid, playlist_id: Option<Uuid>, channel_id: Option<Uuid>) -> Self {
        Self { data: media, id, playlist_id, channel_id }
    }
}

impl DownloadTask {
    /// Creates a new DownloadTask with the given parameters where the itag will get converted into
    /// an any itag to avoid dynamic traits and allow tasks with different itags in the task
    /// handler.
    pub fn new<I: Itag>(itag: I, id: Uuid, video_id: VideoId, playlist_id: Option<Uuid>, channel_id: Option<Uuid>) -> Self {
        Self {
            itag: itag.to_any(),
            id,
            video_id,
            playlist_id,
            channel_id,
        }
    }

    /// Crates a new MediaBrowse that then gets browsed and downlaoded with the itag contained in
    /// the downlaod task.
    /// A FinishedTask gets created from the downloaded media and the already known information like
    /// the channel_id and playlist_id
    /// Returns error if the browse or the download failed
    pub async fn run(self, downloader: Arc<Downloader>) -> Result<FinishedTask> {
        let media = MediaBrowse::new(self.video_id, self.id)
            .browse(downloader)
            .await?
            .download(self.itag, None)
            .await?;

        let finished = FinishedTask::new(media, self.id, self.playlist_id, self.channel_id);
        Ok(finished)
    }
}

impl BundleDownloadTask {
    /// Creates a new BundleDownloadTask with the given parameters
    pub fn new(itags: &'static [AnyItag], id: Uuid, video_id: VideoId, playlist_id: Option<Uuid>, channel_id: Option<Uuid>) -> Self {
        Self { itags, id, video_id, playlist_id, channel_id }
    }

    /// Creates a new MediaBrowse that then gets browsed and bundle downlaoded with the itags contained in
    /// the download bundle task
    /// A FinishedBundleTask gets created from the downlaoded bundle media and the already known
    /// information like the channel_id and playlist_id.
    /// Returns error if the browse or the download failed
    pub async fn run(self, downloader: Arc<Downloader>) -> Result<FinishedBundleTask> {
        let media = MediaBrowse::new(self.video_id, self.id)
            .browse(downloader)
            .await?
            .download_bundle(self.itags, None)
            .await?;

        let finished = FinishedBundleTask::new(media, self.id, self.playlist_id, self.channel_id);

        Ok(finished)
    }
}
