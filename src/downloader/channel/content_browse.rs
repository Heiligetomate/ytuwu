use std::fmt::Debug;

use crate::{
    Dwnlist, Result,
    downloader::{channel::downloaded::DwnChannel, media::downloaded::DwnMedia, media_stream::MediaStream, playlist::browse::PlaylistBrowse},
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
        I: Itag + Copy + Debug,
        I::Stream: Debug + MediaStream,
    {
        let mut downloaded_singles: Vec<DwnMedia<I::Stream>> = Vec::new();
        let mut downloaded_eps: Vec<Dwnlist<I::Stream>> = Vec::new();
        let mut downloaded_albums: Vec<Dwnlist<I::Stream>> = Vec::new();

        for single in self.singles.drain(..) {
            let single = PlaylistBrowse::new(single)
                .browse()
                .await?
                .browse()
                .await?
                .get_song_by_index(0)?
                .download(itag, &None)
                .await?;
            downloaded_singles.push(single);
        }

        for ep in self.eps.drain(..) {
            let ep = PlaylistBrowse::new(ep)
                .browse()
                .await?
                .browse()
                .await?
                .download(itag, &None)
                .await?;
            downloaded_eps.push(ep);
        }

        for album in self.albums.drain(..) {
            let album = PlaylistBrowse::new(album)
                .browse()
                .await?
                .browse()
                .await?
                .download(itag, &None)
                .await?;
            downloaded_albums.push(album);
        }

        Ok(DwnChannel {
            albums: downloaded_albums,
            eps: downloaded_eps,
            singles: downloaded_singles,
        })
    }
}
