use std::{fmt::Debug, sync::Arc};

use uuid::Uuid;

use crate::{
    Downloader, DwnBundleList, DwnMedia, Dwnlist,
    downloader::{
        media::{core::Media, extracted_streams::ThumbRes},
        streams::MediaStream,
    },
    error::{Result, YtuwuError},
    itags::{AnyItag, Itag},
};

const MAX_MEDIA_AT_ONCE: usize = 8;

#[derive(Debug)]
pub struct Playlist {
    title: String,
    media: Vec<Media>,
    id: Uuid,
    downloader: Arc<Downloader>,
}

impl Playlist {
    pub fn new(title: &str, media: Vec<Media>, downloader: Arc<Downloader>) -> Self {
        let id = Uuid::new_v4();
        Self {
            title: title.to_owned(),
            media,
            id,
            downloader,
        }
    }

    fn get_titles(&self) -> Vec<&str> {
        let mut collected = Vec::new();
        for media in self.media.iter() {
            collected.push(media.metadata.title.as_str())
        }
        collected
    }

    pub async fn download<I>(mut self, itag: I, thumb_res: Option<ThumbRes>) -> Result<Dwnlist<I::Stream>>
    where
        I: Itag + Copy + Debug + Send + 'static,
        I::Stream: MediaStream + Debug + Send,
    {
        self.downloader
            .progress_handler
            .on_playlist_started(self.id, self.get_titles());

        let mut downloaded: Vec<DwnMedia<I::Stream>> = Vec::new();

        let mut tasks = Vec::new();

        let semaphore = Arc::new(tokio::sync::Semaphore::new(MAX_MEDIA_AT_ONCE));

        for item in self.media.drain(..) {
            tasks.push(tokio::spawn({
                let sem = Arc::clone(&semaphore);
                let thumb_res = thumb_res.clone();
                async move {
                    let _permit = sem.acquire().await?;
                    item.download(itag, thumb_res).await
                }
            }));
        }

        for task in tasks {
            downloaded.push(task.await??);
        }

        self.downloader
            .progress_handler
            .on_playlist_downloaded(self.id);

        Ok(Dwnlist::new(downloaded, &self.title))
    }

    pub async fn download_streams(mut self, itags: Vec<AnyItag>, thumb_res: Option<ThumbRes>) -> Result<DwnBundleList> {
        let mut downloaded = Vec::new();

        let mut tasks = Vec::new();

        for item in self.media.drain(..) {
            let thumb_res = thumb_res.clone();
            let itags = itags.clone();
            tasks.push(tokio::spawn(item.download_streams(itags, thumb_res)));
        }

        for task in tasks {
            downloaded.push(task.await??);
        }

        Ok(DwnBundleList::new(downloaded, &self.title))
    }

    pub fn get_first(mut self) -> Result<Media> {
        self.media
            .drain(..)
            .nth(0)
            .ok_or(YtuwuError::SongInPlaylistNotFound)
    }
}
