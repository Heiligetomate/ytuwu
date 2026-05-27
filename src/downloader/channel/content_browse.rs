use std::fmt::Debug;

use crate::{
    Dwnlist, Result,
    downloader::{channel::downloaded::DwnChannel, media::downloaded::DwnMedia, playlist::browse::PlaylistBrowse, streams::MediaStream},
    id_resolver::id_types::ChannelPlaylistId,
    itag::Itag,
};

#[derive(Debug, PartialEq, Eq)]
pub struct ChannelContentBrowse {
    pub albums: Vec<ChannelPlaylistId>,
    pub eps: Vec<ChannelPlaylistId>,
    pub singles: Vec<ChannelPlaylistId>,
}

impl ChannelContentBrowse {
    pub async fn download<I>(mut self, itag: I) -> Result<DwnChannel<I::Stream>>
    where
        I: Itag + Copy + Debug + Send + 'static,
        I::Stream: MediaStream + Debug + Send,
    {
        let mut downloaded_singles: Vec<DwnMedia<I::Stream>> = Vec::new();
        let mut downloaded_eps: Vec<Dwnlist<I::Stream>> = Vec::new();
        let mut downloaded_albums: Vec<Dwnlist<I::Stream>> = Vec::new();

        let mut single_tasks = Vec::new();
        let mut ep_tasks = Vec::new();
        let mut album_tasks = Vec::new();

        for single in self.singles.drain(..) {
            let single = PlaylistBrowse::new(single)
                .browse()
                .await?
                .browse()
                .await?
                .get_first();
            match single {
                Ok(media) => single_tasks.push(tokio::spawn(media.download(itag, None))),
                Err(_) => continue,
            }
        }

        for ep in self.eps.drain(..) {
            let ep = PlaylistBrowse::new(ep)
                .browse()
                .await?
                .browse()
                .await?
                .download(itag, None);
            ep_tasks.push(tokio::spawn(ep));
        }

        for album in self.albums.drain(..) {
            let album = PlaylistBrowse::new(album)
                .browse()
                .await?
                .browse()
                .await?
                .download(itag, None);
            album_tasks.push(tokio::spawn(album));
        }

        for task in single_tasks {
            downloaded_singles.push(task.await??);
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
