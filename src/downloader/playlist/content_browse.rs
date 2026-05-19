use crate::{
    Result,
    downloader::{
        media::{browse::MediaBrowse, core::Media},
        playlist::core::Playlist,
    },
};

#[derive(Debug)]
pub struct PlaylistContentBrowse {
    title: String,
    media: Vec<MediaBrowse>,
}

impl PlaylistContentBrowse {
    pub fn new(title: &str, media: Vec<MediaBrowse>) -> Self {
        Self { title: title.to_owned(), media }
    }

    pub async fn browse(mut self) -> Result<Playlist> {
        let mut media_items: Vec<Media> = Vec::new();
        for item in self.media.drain(..) {
            media_items.push(item.browse().await?);
        }
        Ok(Playlist::new(&self.title, media_items))
    }
}
