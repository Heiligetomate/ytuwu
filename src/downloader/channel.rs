use std::fmt::Debug;

use crate::{
    Result,
    downloaded::{DownloadedChannel, RawDownloadedMedia, RawDownloadedPlaylist},
    downloader::{media_stream::MediaStream, playlist::PlaylistBrowse},
    id_resolver::{channel_id::ChannelId, channel_playlist_id::ChannelPlaylistId},
    itag::Itag,
    request::core::captcha_bypass,
};

pub struct ChannelBrowse {
    id: ChannelId,
}

pub struct ChannelContentBrowse {
    pub albums: Vec<ChannelPlaylistId>,
    pub eps: Vec<ChannelPlaylistId>,
    pub singles: Vec<ChannelPlaylistId>,
}

impl ChannelBrowse {
    pub fn new(channel_id: ChannelId) -> Self {
        Self { id: channel_id }
    }

    pub async fn browse(self) -> Result<ChannelContentBrowse> {
        let resp = captcha_bypass(&self.id, 1).await?;
        resp.extract_all_releases()
    }
}

impl ChannelContentBrowse {
    pub async fn download<I>(mut self, itag: I) -> Result<DownloadedChannel<I::Stream>>
    where
        I: Itag + Copy,
        I::Stream: Debug + MediaStream,
    {
        let mut downloaded_singles: Vec<RawDownloadedMedia<I::Stream>> = Vec::new();
        let mut downloaded_eps: Vec<RawDownloadedPlaylist<I::Stream>> = Vec::new();
        let mut downloaded_albums: Vec<RawDownloadedPlaylist<I::Stream>> = Vec::new();

        for single in self.singles.drain(..) {
            let single = PlaylistBrowse::new(single)
                .browse()
                .await?
                .browse()
                .await?
                .get_song_by_index(0)?
                .download_raw(itag)
                .await?;
            downloaded_singles.push(single);
        }

        for ep in self.eps.drain(..) {
            let ep = PlaylistBrowse::new(ep)
                .browse()
                .await?
                .browse()
                .await?
                .download_single_stream(itag)
                .await?;
            downloaded_eps.push(ep);
        }

        for album in self.albums.drain(..) {
            let album = PlaylistBrowse::new(album)
                .browse()
                .await?
                .browse()
                .await?
                .download_single_stream(itag)
                .await?;
            downloaded_albums.push(album);
        }

        Ok(DownloadedChannel {
            albums: downloaded_albums,
            eps: downloaded_eps,
            singles: downloaded_singles,
        })
    }
}
