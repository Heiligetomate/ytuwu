use std::{fmt::Debug, sync::Arc};

use uuid::Uuid;

use crate::{
    Result,
    downloader::{
        Downloader,
        channel::{DwnBundelChannel, DwnChannel},
        media::{DwnBundleMedia, DwnMedia, Media},
        metadata::ChannelMetadata,
        playlist::{DwnBundleList, DwnList, Playlist},
    },
    itags::{AnyItag, Itag},
};

/// This is the final stage before fully downloading a channel. It contains all fully browsed
/// singles, eps and albums, an uuid for identification, a title and an arc downlaoder for shared
/// data.
#[derive(Debug)]
pub struct Channel {
    pub title: String,
    pub downloader: Arc<Downloader>,
    pub singles: Vec<Media>,
    pub albums: Vec<Playlist>,
    pub eps: Vec<Playlist>,
    pub id: Uuid,
}

impl Channel {
    // TODO: Semaphore
    /// Downloads all singles leaving the self with an empty vec for the singles
    /// Returns a vec of DwnMedia with the Stream of the given Itag
    /// Creates a task for every single which gets awaited and collected afterwards
    /// Fails if any of the downloads failed which will cause the singles to stay untouched
    pub async fn download_singles<I>(&mut self, itag: I) -> Result<Vec<DwnMedia<I::Stream>>>
    where
        I: Itag + Copy + 'static,
    {
        let singles = std::mem::take(&mut self.singles);

        let mut tasks = Vec::new();

        for single in singles {
            tasks.push(tokio::spawn(single.download(itag, None)));
        }

        let mut downloaded: Vec<DwnMedia<I::Stream>> = Vec::with_capacity(tasks.len());
        for task in tasks {
            downloaded.push(task.await??);
        }

        Ok(downloaded)
    }

    /// Downloads all singles as bundles leaving the self with an empty vec for the singles
    /// Returns a vec of DwnBundleMedia with all downloaded streams for the itags  
    /// Creates a task for every single which gets awaited and collected afterwards
    /// Fails if any of the downloads failed which will cause the singles to stay untouched
    pub async fn download_bundle_singles(&mut self, itags: &'static [AnyItag]) -> Result<Vec<DwnBundleMedia>> {
        let singles = std::mem::take(&mut self.singles);
        let mut tasks = Vec::new();

        for single in singles {
            tasks.push(tokio::spawn(single.download_bundle(itags, None)));
        }

        let mut downloaded: Vec<DwnBundleMedia> = Vec::with_capacity(tasks.len());

        for task in tasks {
            downloaded.push(task.await??);
        }

        Ok(downloaded)
    }

    /// Downloads all eps leaving the self with an empty vec for the eps
    /// Returns a vec of Dwnlist with the stream of the given itag as stream.
    /// Creates a task for every ep which gets awaited and collected afterwards
    /// Fails if any of the downloads failed which will cause the eps to stay untouched
    pub async fn download_eps<I>(&mut self, itag: I) -> Result<Vec<DwnList<I::Stream>>>
    where
        I: Itag + 'static,
    {
        let eps = std::mem::take(&mut self.eps);
        let len = eps.len();

        let mut tasks = Vec::with_capacity(len);

        for ep in eps {
            tasks.push(tokio::spawn(ep.download(itag, None)));
        }

        let mut downloaded: Vec<DwnList<I::Stream>> = Vec::with_capacity(len);

        for task in tasks {
            downloaded.push(task.await??);
        }

        Ok(downloaded)
    }

    /// Downloads all eps leaving the self with an empty vec for the eps
    /// Returns a vec of DwnBundleList with all downloaded streams for the itags  
    /// Creates a task for every ep which gets awaited and collected afterwards
    /// Fails if any of the downloads failed which will cause the eps to stay untouched
    pub async fn download_bundle_eps(&mut self, itags: &'static [AnyItag]) -> Result<Vec<DwnBundleList>> {
        let eps = std::mem::take(&mut self.eps);
        let len = eps.len();

        let mut tasks = Vec::with_capacity(len);

        for ep in eps {
            tasks.push(tokio::spawn(ep.download_bundle(itags, None)));
        }

        let mut downloaded_eps: Vec<DwnBundleList> = Vec::with_capacity(len);

        for task in tasks {
            downloaded_eps.push(task.await??);
        }

        Ok(downloaded_eps)
    }

    /// Downloads all albums leaving the self with an empty vec for the albums
    /// Returns a vec of Dwnlist with the stream of the given itag as stream.
    /// Creates a task for every album which gets awaited and collected afterwards
    /// Fails if any of the downloads failed which will cause the albums to stay untouched
    pub async fn download_albums<I>(&mut self, itag: I) -> Result<Vec<DwnList<I::Stream>>>
    where
        I: Itag + 'static,
    {
        let albums = std::mem::take(&mut self.albums);
        let len = albums.len();

        let mut tasks = Vec::with_capacity(len);

        for album in albums {
            tasks.push(tokio::spawn(album.download(itag, None)));
        }

        let mut downloaded: Vec<DwnList<I::Stream>> = Vec::with_capacity(len);

        for task in tasks {
            downloaded.push(task.await??);
        }

        Ok(downloaded)
    }

    /// Downloads all albums leaving the self with an empty vec for the albums
    /// Returns a vec of DwnBundleList with all downloaded streams for the itags  
    /// Creates a task for every album which gets awaited and collected afterwards
    /// Fails if any of the downloads failed which will cause the albums to stay untouched
    pub async fn download_bundle_albums(&mut self, itags: &'static [AnyItag]) -> Result<Vec<DwnBundleList>> {
        let albums = std::mem::take(&mut self.albums);
        let len = albums.len();

        let mut tasks = Vec::with_capacity(len);

        for album in albums {
            tasks.push(tokio::spawn(album.download_bundle(itags, None)));
        }

        let mut downloaded: Vec<DwnBundleList> = Vec::with_capacity(len);

        for task in tasks {
            downloaded.push(task.await??);
        }

        Ok(downloaded)
    }

    /// Consumes itself and downloads all singles, all eps and all albums for the given itag and returns a DwnChannel
    /// with the stream of the given itag.
    /// Creates the metadata from the title.
    /// Fails if any download fails.
    pub async fn download<I>(mut self, itag: I) -> Result<DwnChannel<I::Stream>>
    where
        I: Itag + 'static,
    {
        self.downloader
            .progress_handler
            .on_channel_started(self.id, self.singles.len() as u16, self.eps.len() as u16, self.albums.len() as u16, self.title.as_str());
        Ok(DwnChannel {
            albums: self.download_albums(itag).await?,
            eps: self.download_eps(itag).await?,
            singles: self.download_singles(itag).await?,
            metadata: ChannelMetadata::new(self.title.as_str()),
        })
    }

    /// Consumes itself and downloads all singles, all eps and all albums for all given itags and returns a DwnBundelChanne
    /// with the streams of the given itags.
    /// Creates the metadata from the title.
    /// Fails if any download fails.
    pub async fn download_bundle(mut self, itags: &'static [AnyItag]) -> Result<DwnBundelChannel> {
        Ok(DwnBundelChannel {
            albums: self
                .download_bundle_albums(itags)
                .await?,
            eps: self.download_bundle_eps(itags).await?,
            singles: self
                .download_bundle_singles(itags)
                .await?,
            metadata: ChannelMetadata::new(self.title.as_str()),
        })
    }
}

// TODO: redundant code
