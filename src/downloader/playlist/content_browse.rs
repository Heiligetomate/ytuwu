use std::sync::Arc;

use uuid::Uuid;

use crate::{
    Result,
    downloader::{
        Downloader,
        media::{Media, MediaBrowse},
        playlist::core::Playlist,
    },
    error::YtuwuError,
    id_resolver::Id,
    itags::AnyItag,
};

/// This struct is an inbetween stage for fully browsing a playlist which is needed because a
/// playlist has to be browsed with the given browseid first to get the collection of video ids
/// After that those video ids have to be browsed to get a vec of media that can then be downloaded
/// This struct holds the title of the playlist, the downloader to share data, an id for
/// identification and a vec of mediabrowses all contain a video id
#[derive(Debug)]
pub struct PlaylistContentBrowse {
    pub id: Uuid,
    pub title: String,
    pub media: Vec<MediaBrowse>,
    pub downloader: Arc<Downloader>,
}

impl PlaylistContentBrowse {
    /// Creates a new PlaylistContentBrowse with the given data
    /// Takes ownership of the title by calling .to_owned
    pub fn new(title: &str, media: Vec<MediaBrowse>, downloader: Arc<Downloader>, id: Uuid) -> Self {
        Self {
            id,
            title: title.to_owned(),
            media,
            downloader,
        }
    }

    // TODO: This does not have to return result i think
    // TODO: There are many list titles inserted
    /// Consumes itself, pushes the list title to the downlaoder storage with the own id as key and
    /// pushes every media as a task to the task handler with the playlist id for identification and
    /// mapping later.
    pub async fn add_tasks(mut self, itag: AnyItag) -> Result<()> {
        for media in self.media.drain(..) {
            let video_id = media.video_id;
            let id = media.id;

            self.downloader
                .storage
                .lock()
                .await
                .insert_list_title(self.id, &self.title);

            self.downloader
                .task_handler
                .lock()
                .await
                .push(video_id, Some(self.id), None, id, itag);
        }

        Ok(())
    }

    // TODO: Is the result as return type needed here?
    // TODO: Nothing failes here, evreyting failed gets skipped which could cause empty lists
    /// Consumes itself and browses every video id of itself so that there is a vec of media created
    /// Creates a task pool and pushes tokio tasks for brwosing the video ids for each mediabrowse
    /// in self.
    /// Awaits all tasks right after all tasks are created and collects all browsed medias
    /// Creates a new playlist with the already existing downloader and title and the freshly
    /// browsed media.
    /// Skips any media that was failed to browse which can happen when there is an age vericiation
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

    /// Consumes itself and returns the first element in media as MediaBrowse
    /// This is useful for singles that contain only one song (not sure if thats actually that case)
    /// Fails if there is no element contained in self.media
    pub fn first(self) -> Result<MediaBrowse> {
        let len = self.media.len() as u8;
        self.media
            .into_iter()
            .next()
            .ok_or(YtuwuError::MediaNotContained(len, 0))
    }
}
