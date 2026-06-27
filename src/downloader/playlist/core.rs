use std::{fmt::Debug, sync::Arc};

use uuid::Uuid;

use crate::{
    downloader::{
        Downloader,
        media::{DwnMedia, Media, ThumbRes},
        playlist::{DwnBundleList, Dwnlist},
    },
    error::{Result, YtuwuError},
    itags::{AnyItag, Itag},
};

/// This controls how many medias there should be downloaded concurrently
/// Should not be too many at once because youtube might rate limit
const MAX_MEDIA_AT_ONCE: usize = 8;

/// This struct is the pre stage for downloading the full playlist.
/// It contains a vec of already browsed media, the title of the playlist, an arc downloader for
/// shared data and an uuid for identification
#[derive(Debug)]
pub struct Playlist {
    title: String,
    media: Vec<Media>,
    id: Uuid,
    downloader: Arc<Downloader>,
}

impl Playlist {
    /// TODO: Why is there a new uuid generated here? major bug
    /// Creates a new playlist from the given parameters.
    /// Takes ownership if the title.
    pub fn new(title: &str, media: Vec<Media>, downloader: Arc<Downloader>) -> Self {
        let id = Uuid::new_v4();
        Self {
            title: title.to_owned(),
            media,
            id,
            downloader,
        }
    }

    /// Returns a vec with all titles of the browsed medias in self.media
    pub fn get_titles(&self) -> Vec<&str> {
        let mut collected = Vec::new();
        for media in self.media.iter() {
            collected.push(media.metadata.title.as_str())
        }
        collected
    }

    // TODO: Maybe add an option to just download the playlist thumbnail?
    /// Consumes itself and returns a DwnList with the Stream of the given itag
    /// Creates a task for each media in self.media and collects them all in a vec of tasks which
    /// then get awaited with a semaphore afterwards using the max tasks const.
    /// Downloads a thumbnail for each media if there was a thumbnail resolution given
    /// Creates a DwnPlaylist with the downloaded media and the already existing metadata
    /// Fails if any of the tasks failed to download or the acquiring of the semaphore failed.
    pub async fn download<I>(mut self, itag: I, thumb_res: Option<ThumbRes>) -> Result<Dwnlist<I::Stream>>
    where
        I: Itag + 'static,
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

    // TODO: Has to use the semaphore for limiting the concurrent downloads
    /// Consumes itself and returns a DwnBundleList.
    /// Downloads all medias in self.media as a bundle meaning the streams for all the itags get
    /// downlaoded. Downloads a thumbnail for every media if thumb_res is not None.
    /// Creates a tokio spawn task for every media, awaits them afterwards and collectts them
    /// After that the DwnBundleList gets created with the already existing metadata and the
    /// downloaded bundle media
    /// Fails if any of the downloads for the media failed
    pub async fn download_bundle(mut self, itags: &[AnyItag], thumb_res: Option<ThumbRes>) -> Result<DwnBundleList> {
        let mut downloaded = Vec::new();

        let mut tasks = Vec::new();

        let itags: Arc<[AnyItag]> = itags.into();

        for item in self.media.drain(..) {
            let thumb_res = thumb_res.clone();
            let itags = Arc::clone(&itags);
            tasks.push(tokio::spawn(async move {
                item.download_bundle(&itags, thumb_res)
                    .await
            }));
        }

        for task in tasks {
            downloaded.push(task.await??);
        }

        Ok(DwnBundleList::new(downloaded, &self.title))
    }

    // TODO: not needed i think
    /// Consumes itself and returns the first element in self.media
    /// Returns Err if self.media is empty
    pub fn first(mut self) -> Result<Media> {
        self.media
            .drain(..)
            .next()
            .ok_or(YtuwuError::SongInPlaylistNotFound)
    }
}
