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

/// The task handler is one of the main fields on the downloader
/// Its task is to store all tasks that have to be worked
/// This includes downloading media and bundle media
/// It also holds a Semaphore to ensure that only a specific number of tasks run parallel
/// The basic funciton is to push tasks and work tasks. As soon as the tasks get worked, they are
/// getting moved into the downloader storage.
#[derive(Debug)]
pub struct TaskHandler {
    tasks: Vec<DownloadTask>,
    bundle_tasks: Vec<BundleDownloadTask>,
    limit: Arc<Semaphore>,
}

impl Default for TaskHandler {
    /// New default task handler containign two empty task lists and a semaphore with the MAX_TASKS
    /// as limit
    fn default() -> Self {
        Self {
            tasks: Vec::new(),
            bundle_tasks: Vec::new(),
            limit: Arc::new(Semaphore::new(MAX_TASKS)),
        }
    }
}

impl TaskHandler {
    /// Pushed one task on the TaskHandler tasks vec
    pub fn push(&mut self, video_id: VideoId, playlist_id: Option<Uuid>, channel_id: Option<Uuid>, id: Uuid, itag: AnyItag) {
        let task = DownloadTask::new(itag, id, video_id, playlist_id, channel_id);
        self.tasks.push(task);
    }

    /// Pushes one bundle task on the TaskHandler bundle vec
    pub fn push_bundle(&mut self, video_id: VideoId, playlist_id: Option<Uuid>, channel_id: Option<Uuid>, id: Uuid, itags: &'static [AnyItag]) {
        let task = BundleDownloadTask::new(itags, id, video_id, playlist_id, channel_id);
        self.bundle_tasks.push(task);
    }

    /// Works all bundle tasks and all normal tasks where only MAX_TASKS are running parallel
    /// All results that were not successful will just be ignored
    /// Pushes all BundleDownloadTasks and DownloadTask to the downloader storage where they can
    /// then be extracted with the correct ids.
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

        storage.push_vec(results);
        storage.push_bundle_vec(bundle_results);
    }
}
