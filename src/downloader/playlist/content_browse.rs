use crate::{
    Id, Result,
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
            let video_id = item.video_id.as_str().to_string();
            let browsed_item = match item.browse().await {
                Ok(item) => item,
                Err(_) => {
                    println!("Following item was skipped: {}", video_id);
                    continue;
                }
            };
            media_items.push(browsed_item);
        }
        Ok(Playlist::new(&self.title, media_items))
    }
}
