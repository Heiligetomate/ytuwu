use std::{fmt::Debug, sync::Arc};

use tokio::sync::Semaphore;
use uuid::Uuid;

use crate::{Downloader, DwnBundleMedia, DwnMedia, Result, downloader::media::browse::MediaBrowse, itags::AnyItag, streams::AnyStream, types::VideoId};

const MAX_TASKS: usize = 4;

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
    fn new(media: DwnMedia<AnyStream>, id: Uuid, playlist_id: Option<Uuid>, channel_id: Option<Uuid>) -> Self {
        Self {
            data: media,
            id,
            playlist_id,
            channel_id,
        }
    }
}

impl FinishedBundleTask {
    fn new(media: DwnBundleMedia, id: Uuid, playlist_id: Option<Uuid>, channel_id: Option<Uuid>) -> Self {
        Self {
            data: media,
            id,
            playlist_id,
            channel_id,
        }
    }
}

impl DownloadTask {
    async fn run(self, downloader: Arc<Downloader>) -> Result<FinishedTask> {
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
    async fn run(self, downloader: Arc<Downloader>) -> Result<FinishedBundleTask> {
        let media = MediaBrowse::new(self.video_id, self.id)
            .browse(downloader)
            .await?
            .download_bundle(self.itags, None)
            .await?;

        let finished = FinishedBundleTask::new(media, self.id, self.playlist_id, self.channel_id);

        Ok(finished)
    }
}

#[derive(Debug)]
pub struct TaskHandler {
    tasks: Vec<DownloadTask>,
    bundle_tasks: Vec<BundleDownloadTask>,
    limit: Arc<Semaphore>,
}

impl Default for TaskHandler {
    fn default() -> Self {
        Self {
            tasks: Vec::new(),
            bundle_tasks: Vec::new(),
            limit: Arc::new(Semaphore::new(MAX_TASKS)),
        }
    }
}

impl TaskHandler {
    pub fn push(&mut self, video_id: VideoId, playlist_id: Option<Uuid>, channel_id: Option<Uuid>, id: Uuid, itag: AnyItag) {
        let task = DownloadTask {
            video_id,
            playlist_id,
            channel_id,
            id,
            itag: itag,
        };
        self.tasks.push(task);
    }

    pub fn push_bundle(&mut self, video_id: VideoId, playlist_id: Option<Uuid>, channel_id: Option<Uuid>, id: Uuid, itags: &'static [AnyItag]) {
        let task = BundleDownloadTask {
            video_id,
            playlist_id,
            channel_id,
            id,
            itags,
        };
        self.bundle_tasks.push(task);
    }

    pub async fn work(self, downloader: Arc<Downloader>) {
        // TODO: error handling, return a result?

        let mut handles = Vec::new();
        let mut bundle_handles = Vec::new();

        for task in self.tasks {
            let downloader = Arc::clone(&downloader);
            let limit = Arc::clone(&self.limit);
            handles.push(tokio::spawn(async move {
                let _permit = limit.acquire().await.unwrap();
                task.run(downloader).await
            }));
        }

        for task in self.bundle_tasks {
            let downloader = Arc::clone(&downloader);
            let limit = Arc::clone(&self.limit);
            bundle_handles.push(tokio::spawn(async move {
                let _permit = limit.acquire().await.unwrap();
                task.run(downloader).await
            }));
        }

        let mut results = Vec::new();

        for handle in handles {
            if let Ok(Ok(media)) = handle.await {
                results.push(media);
            }
        }

        let mut bundle_results = Vec::new();

        for handle in bundle_handles {
            if let Ok(Ok(bundle_media)) = handle.await {
                bundle_results.push(bundle_media);
            }
        }

        let mut storage = downloader.downloaded.lock().await;

        storage.push_any_vec(results);
        storage.push_bundle_vec(bundle_results);
    }
}
