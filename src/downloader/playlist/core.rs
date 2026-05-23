use std::fmt::Debug;

use crate::{
    DwnBundleList, DwnMedia, Dwnlist,
    downloader::{
        media::{core::Media, extracted_streams::ThumbRes},
        media_stream::MediaStream,
    },
    error::{Result, YtuwuError},
    itag::AnyItag,
    models::itag::Itag,
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

    pub async fn download<I>(mut self, itag: I, thumb_res: &Option<ThumbRes>) -> Result<Dwnlist<I::Stream>>
    where
        I: Itag + Copy + Debug,
        I::Stream: MediaStream + Debug,
    {
        let mut downloaded: Vec<DwnMedia<I::Stream>> = Vec::new();

        for item in self.media.drain(..) {
            let downloaded_media = item.download(itag, thumb_res).await?;
            downloaded.push(downloaded_media);
        }

        Ok(Dwnlist::new(downloaded, &self.title))
    }

    pub async fn download_streams(mut self, itags: Vec<AnyItag>, thumb_res: &Option<ThumbRes>) -> Result<DwnBundleList> {
        let mut downloaded = Vec::new();

        for item in self.media.drain(..) {
            downloaded.push(
                item.download_streams(&itags, thumb_res)
                    .await?,
            );
        }

        Ok(DwnBundleList::new(downloaded, &self.title))
    }

    pub fn get_song_by_index(&self, index: usize) -> Result<&Media> {
        self.media
            .get(index)
            .ok_or(YtuwuError::SongInPlaylistNotFound)
    }
}
