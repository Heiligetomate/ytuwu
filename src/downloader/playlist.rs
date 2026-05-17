use std::fmt::Debug;

use serde::de::DeserializeOwned;

use crate::{
    downloaded::DownloadedMedia,
    downloader::{
        downloaded::DownloadedPlaylist,
        media::{Media, MediaBrowse},
        thumbnail::PlaylistThumbnail,
    },
    error::{Result, YtuwuError},
    id_resolver::{
        channel_playlist_id::ChannelPlaylistId,
        id::{BrowseId, Id},
    },
    models::{itag::Itag, player::ThumbnailResolution, response::BrowseResponse, slow_browse::SlowBrowseResponse},
    name_trimmer,
    request::{clients::client::ClientWithHeaders, core::captcha_bypass},
};

#[derive(Debug)]
pub struct PlaylistBrowse<B: BrowseId> {
    browse_id: B,
}

#[derive(Debug)]
pub struct PlaylistContentBrowse {
    title: String,
    media: Vec<MediaBrowse>,
}

#[derive(Debug)]
pub struct Playlist {
    title: String,
    media: Vec<Media>,
}

impl<B: BrowseId> PlaylistBrowse<B>
where
    <<B as Id>::Client as ClientWithHeaders>::Response: DeserializeOwned + Debug,
    <<B as Id>::Client as ClientWithHeaders>::Response: BrowseResponse,
{
    pub fn new(id: B) -> Self {
        Self { browse_id: id }
    }
    pub async fn browse(self) -> Result<PlaylistContentBrowse> {
        println!("{}", self.browse_id.as_str());
        let test_id = ChannelPlaylistId::new("MPREb_ZFVkxH6MkHf");
        let response: SlowBrowseResponse = captcha_bypass(&test_id, 1).await?;
        //let response = captcha_bypass(&self.browse_id, 2).await?;
        let mut ids = response.get_video_ids()?;
        let title = response.get_album_title()?.to_owned();
        let trimmed_title = name_trimmer::trim(title, "-");
        let media: Vec<MediaBrowse> = ids
            .drain(..)
            .map(|id| MediaBrowse::new(id))
            .collect();
        Ok(PlaylistContentBrowse { title: trimmed_title, media })
    }
}

impl PlaylistContentBrowse {
    pub async fn browse(mut self) -> Result<Playlist> {
        let mut media_items: Vec<Media> = Vec::new();
        for item in self.media.drain(..) {
            media_items.push(item.browse().await?);
        }
        Ok(Playlist { media: media_items, title: self.title })
    }
}

impl Playlist {
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
