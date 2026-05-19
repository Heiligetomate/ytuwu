use std::fmt::Debug;

use crate::{
    downloader::{
        media::{
            core::Media,
            downloaded::{DownloadedMedia, RawDownloadedMedia},
        },
        media_stream::MediaStream,
        playlist::downloaded::{DownloadedPlaylist, RawDownloadedPlaylist},
        thumbnail::PlaylistThumbnail,
    },
    error::{Result, YtuwuError},
    models::{itag::Itag, player::ThumbnailResolution},
};

#[derive(Debug)]
pub struct Playlist {
    title: String,
    media: Vec<Media>,
}

impl Playlist {
    pub fn new(title: &str, media: Vec<Media>) -> Self {
        Self { title: title.to_owned(), media }
    }

    pub async fn download_single_stream<I>(mut self, itag: I) -> Result<RawDownloadedPlaylist<I::Stream>>
    where
        I: Itag + Copy,
        I::Stream: MediaStream + Debug,
    {
        let mut downloaded: Vec<RawDownloadedMedia<I::Stream>> = Vec::new();

        for item in self.media.drain(..) {
            let downloaded_media = item.download_raw(itag).await?;
            downloaded.push(downloaded_media);
        }

        Ok(RawDownloadedPlaylist::new(downloaded))
    }

    pub async fn download_full<I>(mut self, itag: I, thumbnail_resolution: ThumbnailResolution) -> Result<DownloadedPlaylist<I::Stream>>
    where
        I: Itag + Copy + Debug,
        I::Stream: Debug,
    {
        let mut downloaded: Vec<DownloadedMedia<I::Stream>> = Vec::new();

        for item in self.media.drain(..) {
            let downloaded_media = item
                .download_full(itag, &thumbnail_resolution)
                .await?;
            downloaded.push(downloaded_media);
        }
        Ok(DownloadedPlaylist::new(&self.title, downloaded))
    }

    pub async fn download_thumbnails(&self, thumbnail_resolution: ThumbnailResolution) -> Result<PlaylistThumbnail> {
        let mut thumbnails = Vec::new();
        for item in self.media.iter() {
            let downloaded_thumbnail = item
                .download_thumbnail(&thumbnail_resolution)
                .await?;
            thumbnails.push(downloaded_thumbnail);
        }
        Ok(PlaylistThumbnail::new(thumbnails))
    }

    pub fn get_song_by_index(&self, index: usize) -> Result<&Media> {
        self.media
            .get(index)
            .ok_or(YtuwuError::SongInPlaylistNotFound)
    }
}
