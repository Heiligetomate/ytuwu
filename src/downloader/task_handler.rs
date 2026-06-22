use std::{fmt::Debug, sync::Arc};

use tokio::sync::Semaphore;
use uuid::Uuid;

use crate::{
    downloader::{
        Downloader,
        tasks::{BundleDownloadTask, DownloadTask},
    },
    itags::AnyItag,
    types::VideoId,
};

/// This defines how many downlaod tasks can run parallel
/// Do not set this too high because youtube will rate limit you if this is too high which causes
/// slower downloads or even full abortion of the download
const MAX_TASKS: usize = 4;

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
        let task = DownloadTask::new(itag, id, video_id, playlist_id, channel_id);
        self.tasks.push(task);
    }

    pub fn push_bundle(&mut self, video_id: VideoId, playlist_id: Option<Uuid>, channel_id: Option<Uuid>, id: Uuid, itags: &'static [AnyItag]) {
        let task = BundleDownloadTask::new(itags, id, video_id, playlist_id, channel_id);
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

        let mut storage = downloader.storage.lock().await;

        storage.push_any_vec(results);
        storage.push_bundle_vec(bundle_results);
    }
}
