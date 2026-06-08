use std::sync::Arc;

use uuid::Uuid;

use crate::{
    Downloader, Id, Result,
    downloader::{
        media::{browse::MediaBrowse, core::Media},
        playlist::core::Playlist,
    },
};

#[derive(Debug)]
pub struct PlaylistContentBrowse {
    pub(super) id: Uuid,
    pub(super) title: String,
    pub(super) media: Vec<MediaBrowse>,
    pub(super) downloader: Arc<Downloader>,
}

impl PlaylistContentBrowse {
    pub fn new(title: &str, media: Vec<MediaBrowse>, downloader: Arc<Downloader>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.to_owned(),
            media,
            downloader,
        }
    }

    pub async fn add_tasks(mut self) -> Result<()> {
        for media in self.media.drain(..) {
            let id = media.video_id;
            self.downloader
                .task_handler
                .lock()
                .await
                .push(id, Some(self.id), None, Uuid::new_v4());
        }

        Ok(())
    }

    pub async fn browse(mut self) -> Result<Playlist> {
        let mut media_items: Vec<Media> = Vec::new();
        let mut tasks = Vec::new();

        for item in self.media.drain(..) {
            let downloader = Arc::clone(&self.downloader);
            tasks.push(tokio::spawn(async move {
                let id = item.video_id.as_str().to_string();
                item.browse(downloader)
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
        Ok(Playlist::new(&self.title, media_items, self.downloader))
    }
}
