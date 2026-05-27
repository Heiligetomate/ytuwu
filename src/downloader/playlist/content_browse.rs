use std::sync::Arc;

use crate::{
    Id, Result,
    downloader::{
        core::SharedVd,
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

    pub async fn browse(mut self, vd: &SharedVd) -> Result<Playlist> {
        let mut media_items: Vec<Media> = Vec::new();
        let mut tasks = Vec::new();

        for item in self.media.drain(..) {
            let vd_cloned = Arc::clone(vd);
            tasks.push(tokio::spawn(async move {
                let id = item.video_id.as_str().to_string();
                item.browse(&vd_cloned)
                    .await
                    .map_err(|e| (id, e))
            }));
        }

        for task in tasks {
            let media = match task.await? {
                Ok(item) => item,
                Err((id, _)) => {
                    println!("Following item was skipped: {}", id);
                    continue;
                }
            };
            media_items.push(media);
        }
        Ok(Playlist::new(&self.title, media_items))
    }
}
