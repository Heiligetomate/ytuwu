use std::{fmt::Debug, sync::Arc};

use crate::{
    Downloader, Dwnlist, Result,
    downloader::{channel::downloaded::DwnChannel, media::downloaded::DwnMedia, playlist::browse::PlaylistBrowse, streams::MediaStream},
    id_resolver::types::{BrowseId, ChannelPlaylistId},
    itags::Itag,
};

#[derive(Debug)]
pub struct ChannelContentBrowse {
    pub downloader: Arc<Downloader>,
    pub albums: Vec<ChannelPlaylistId>,
    pub eps: Vec<ChannelPlaylistId>,
    pub singles: Vec<ChannelPlaylistId>,
}

impl ChannelContentBrowse {
    pub async fn download_singles<I>(&self, itag: I) -> Result<Vec<DwnMedia<I::Stream>>>
    where
        I: Itag + Copy + Debug + Send + 'static,
        I::Stream: MediaStream + Debug + Send,
    {
        let mut browse_tasks = Vec::new();
        let mut content_browse_tasks = Vec::new();
        let mut download_tasks = Vec::new();
        let mut downloaded: Vec<DwnMedia<I::Stream>> = Vec::new();

        for single in self.singles.iter() {
            let downloader = Arc::clone(&self.downloader);
            browse_tasks.push(tokio::spawn(PlaylistBrowse::new(BrowseId::ChannelBrowseId(single.clone()), downloader).browse()));
        }

        let mut browse_results = Vec::new();
        for task in browse_tasks {
            browse_results.push(task.await??);
        }

        for result in browse_results {
            content_browse_tasks.push(tokio::spawn(async move { result.browse().await }));
        }

        let mut content_browse_results = Vec::new();
        for task in content_browse_tasks {
            content_browse_results.push(task.await??.get_first()?);
        }

        for media in content_browse_results {
            download_tasks.push(tokio::spawn(media.download(itag, None)));
        }

        for task in download_tasks {
            downloaded.push(task.await??);
        }

        Ok(downloaded)
    }

    pub async fn download<I>(mut self, itag: I) -> Result<DwnChannel<I::Stream>>
    where
        I: Itag + Copy + Debug + Send + 'static,
        I::Stream: MediaStream + Debug + Send,
    {
        let downloaded_singles: Vec<DwnMedia<I::Stream>> = self.download_singles(itag).await?;
        let mut downloaded_eps: Vec<Dwnlist<I::Stream>> = Vec::new();
        let mut downloaded_albums: Vec<Dwnlist<I::Stream>> = Vec::new();

        let mut ep_tasks = Vec::new();
        let mut album_tasks = Vec::new();

        for ep in self.eps.drain(..) {
            let downloader = Arc::clone(&self.downloader);
            let ep = PlaylistBrowse::new(BrowseId::ChannelBrowseId(ep), downloader)
                .browse()
                .await?
                .browse()
                .await?
                .download(itag, None);
            ep_tasks.push(tokio::spawn(ep));
        }

        for album in self.albums.drain(..) {
            let downloader = Arc::clone(&self.downloader);
            let album = PlaylistBrowse::new(BrowseId::ChannelBrowseId(album), downloader)
                .browse()
                .await?
                .browse()
                .await?
                .download(itag, None);
            album_tasks.push(tokio::spawn(album));
        }

        for task in ep_tasks {
            downloaded_eps.push(task.await??);
        }

        for task in album_tasks {
            downloaded_albums.push(task.await??);
        }

        Ok(DwnChannel {
            albums: downloaded_albums,
            eps: downloaded_eps,
            singles: downloaded_singles,
        })
    }
}
