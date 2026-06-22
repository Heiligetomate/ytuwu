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

#[derive(Debug)]
pub struct DownloadTask {
    itag: AnyItag,
    id: Uuid,
    video_id: VideoId,
    playlist_id: Option<Uuid>,
    channel_id: Option<Uuid>,
}

#[derive(Debug)]
pub struct BundleDownloadTask {
    id: Uuid,
    video_id: VideoId,
    playlist_id: Option<Uuid>,
    channel_id: Option<Uuid>,
    itags: &'static [AnyItag],
}

#[derive(Debug)]
pub struct FinishedTask {
    pub data: DwnMedia<AnyStream>,
    pub id: Uuid,
    pub playlist_id: Option<Uuid>,
    pub channel_id: Option<Uuid>,
}

#[derive(Debug)]
pub struct FinishedBundleTask {
    pub data: DwnBundleMedia,
    pub id: Uuid,
    pub playlist_id: Option<Uuid>,
    pub channel_id: Option<Uuid>,
}

impl FinishedTask {
    pub fn new(media: DwnMedia<AnyStream>, id: Uuid, playlist_id: Option<Uuid>, channel_id: Option<Uuid>) -> Self {
        Self { data: media, id, playlist_id, channel_id }
    }
}

impl FinishedBundleTask {
    pub fn new(media: DwnBundleMedia, id: Uuid, playlist_id: Option<Uuid>, channel_id: Option<Uuid>) -> Self {
        Self { data: media, id, playlist_id, channel_id }
    }
}

impl DownloadTask {
    pub fn new<I: Itag>(itag: I, id: Uuid, video_id: VideoId, playlist_id: Option<Uuid>, channel_id: Option<Uuid>) -> Self {
        Self {
            itag: itag.to_any(),
            id,
            video_id,
            playlist_id,
            channel_id,
        }
    }

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
    pub fn new(itags: &'static [AnyItag], id: Uuid, video_id: VideoId, playlist_id: Option<Uuid>, channel_id: Option<Uuid>) -> Self {
        Self { itags, id, video_id, playlist_id, channel_id }
    }

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
