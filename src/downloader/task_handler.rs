use std::{fmt::Debug, sync::Arc};

use tokio::sync::Semaphore;
use uuid::Uuid;

use crate::{Downloader, DwnMedia, Result, downloader::media::browse::MediaBrowse, itags::Itag, types::VideoId};

const MAX_TASKS: usize = 4;

#[derive(Debug)]
pub struct DownloadTask {
    id: Uuid,
    video_id: VideoId,
    playlist_id: Option<Uuid>,
    channel_id: Option<Uuid>,
}

impl DownloadTask {
    async fn run<I: Itag>(self, downloader: Arc<Downloader>, itag: I) -> Result<DwnMedia<I::Stream>>
where {
        let media = MediaBrowse::new(self.video_id, self.id)
            .browse(downloader)
            .await?
            .download(itag, None)
            .await?;
        Ok(media)
    }
}

#[derive(Debug)]
pub struct TaskHandler {
    tasks: Vec<DownloadTask>,
    limit: Arc<Semaphore>,
}

impl Default for TaskHandler {
    fn default() -> Self {
        Self {
            tasks: Vec::new(),
            limit: Arc::new(Semaphore::new(MAX_TASKS)),
        }
    }
}

impl TaskHandler {
    pub fn push(&mut self, video_id: VideoId, playlist_id: Option<Uuid>, channel_id: Option<Uuid>, id: Uuid) {
        let task = DownloadTask { video_id, playlist_id, channel_id, id };
        self.tasks.push(task);
    }

    pub async fn work<I>(self, downloader: Arc<Downloader>, itag: I) -> Vec<DwnMedia<I::Stream>>
    where
        I: Itag + 'static,
        I::Stream: 'static,
    {
        let mut handles = Vec::new();
        for task in self.tasks {
            let downloader = Arc::clone(&downloader);
            let limit = Arc::clone(&self.limit);
            handles.push(tokio::spawn(async move {
                let _permit = limit.acquire().await.unwrap();
                task.run(downloader, itag).await
            }));
        }

        let mut results = Vec::new();
        for handle in handles {
            if let Ok(Ok(media)) = handle.await {
                results.push(media);
            }
        }
        results
    }
}
