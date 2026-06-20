use std::sync::Arc;

use uuid::Uuid;

use crate::{
    Downloader, Result,
    downloader::{
        channel::core::Channel,
        media::{Media, MediaBrowse},
        metadata::ChannelMetadata,
        playlist::{Playlist, PlaylistBrowse, PlaylistContentBrowse},
        store::ChannelTemplate,
    },
    itags::AnyItag,
    types::{BrowseId, ChannelPlaylistId},
};

#[derive(Debug)]
pub struct ChannelContentBrowse {
    pub title: String,
    pub downloader: Arc<Downloader>,
    pub albums: Vec<ChannelPlaylistId>,
    pub eps: Vec<ChannelPlaylistId>,
    pub singles: Vec<ChannelPlaylistId>,
    pub id: Uuid,
}

impl ChannelContentBrowse {
    pub async fn add_tasks(self, itag: AnyItag) -> Result<()> {
        let mut task_pool = self
            .downloader
            .task_handler
            .lock()
            .await;

        let singles = self.half_browse_singles().await?;
        let eps = self.half_browse_eps().await?;
        let albums = self.half_browse_albums().await?;

        let mut single_ids = Vec::with_capacity(self.singles.len());
        let mut ep_ids = Vec::with_capacity(self.eps.len());
        let mut album_ids = Vec::with_capacity(self.albums.len());

        let channel_id = Some(self.id);

        for single in singles {
            single_ids.push(single.id);
            task_pool.push(single.video_id, None, channel_id, single.id, itag);
        }

        for ep in eps {
            ep_ids.push(ep.id);
            let ep_id = Some(ep.id);
            for media in ep.media {
                task_pool.push(media.video_id, ep_id, channel_id, media.id, itag);
            }
        }

        for album in albums {
            album_ids.push(album.id);
            let album_id = Some(album.id);
            for media in album.media {
                task_pool.push(media.video_id, album_id, channel_id, media.id, itag);
            }
        }

        let metadata = ChannelMetadata::new(&self.title);
        let template = ChannelTemplate::new(metadata, ep_ids, album_ids, single_ids);

        self.downloader
            .storage
            .lock()
            .await
            .insert_channel_template(self.id, template);

        Ok(())
    }

    pub async fn add_bundle_tasks(self, itags: &'static [AnyItag]) -> Result<()> {
        let mut task_pool = self
            .downloader
            .task_handler
            .lock()
            .await;

        let singles = self.half_browse_singles().await?;
        let eps = self.half_browse_eps().await?;
        let albums = self.half_browse_albums().await?;

        let mut single_ids = Vec::with_capacity(self.singles.len());
        let mut ep_ids = Vec::with_capacity(self.eps.len());
        let mut album_ids = Vec::with_capacity(self.albums.len());

        let channel_id = Some(self.id);

        for single in singles {
            single_ids.push(single.id);
            task_pool.push_bundle(single.video_id, None, channel_id, single.id, itags);
        }

        for ep in eps {
            ep_ids.push(ep.id);
            let ep_id = Some(ep.id);
            for media in ep.media {
                task_pool.push_bundle(media.video_id, ep_id, channel_id, media.id, itags);
            }
        }

        for album in albums {
            album_ids.push(album.id);
            let album_id = Some(album.id);
            for media in album.media {
                task_pool.push_bundle(media.video_id, album_id, channel_id, media.id, itags);
            }
        }

        let metadata = ChannelMetadata::new(&self.title);
        let template = ChannelTemplate::new(metadata, ep_ids, album_ids, single_ids);

        self.downloader
            .storage
            .lock()
            .await
            .insert_channel_template(self.id, template);

        Ok(())
    }

    async fn half_browse_singles(&self) -> Result<Vec<MediaBrowse>> {
        let mut browse_tasks = Vec::new();

        for single in self.singles.iter() {
            let downloader = Arc::clone(&self.downloader);
            browse_tasks.push(tokio::spawn(PlaylistBrowse::new(BrowseId::ChannelBrowseId(single.clone()), downloader, Uuid::new_v4()).browse()));
        }

        let mut browse_results = Vec::with_capacity(browse_tasks.len());

        for task in browse_tasks {
            browse_results.push(task.await??.first()?);
        }

        Ok(browse_results)
    }

    pub async fn browse_singles(&self) -> Result<Vec<Media>> {
        let browse_results = self.half_browse_singles().await?;

        let mut content_browse_tasks = Vec::with_capacity(browse_results.len());

        for media_browse in browse_results {
            let downloader = Arc::clone(&self.downloader);
            content_browse_tasks.push(tokio::spawn(media_browse.browse(downloader)));
        }

        let mut singles_result = Vec::with_capacity(content_browse_tasks.len());

        for task in content_browse_tasks {
            singles_result.push(task.await??);
        }

        Ok(singles_result)
    }

    async fn half_browse_eps(&self) -> Result<Vec<PlaylistContentBrowse>> {
        // TODO: Dont clone
        let mut tasks = Vec::with_capacity(self.eps.len());

        for ep in self.eps.iter() {
            let downloader = Arc::clone(&self.downloader);
            let ep = PlaylistBrowse::new(BrowseId::ChannelBrowseId(ep.clone()), downloader, Uuid::new_v4()).browse();
            tasks.push(tokio::spawn(ep));
        }

        let mut browsed = Vec::with_capacity(tasks.len());

        for task in tasks {
            browsed.push(task.await??);
        }

        Ok(browsed)
    }

    pub async fn browse_eps(&self) -> Result<Vec<Playlist>> {
        let mut ep_tasks = Vec::new();

        let half = self.half_browse_eps().await?;

        for ep in half {
            ep_tasks.push(tokio::spawn(ep.browse()));
        }

        let mut browsed_eps = Vec::with_capacity(ep_tasks.len());

        for task in ep_tasks {
            browsed_eps.push(task.await??);
        }

        Ok(browsed_eps)
    }

    async fn half_browse_albums(&self) -> Result<Vec<PlaylistContentBrowse>> {
        // TODO: Dont clone
        let mut tasks = Vec::with_capacity(self.eps.len());

        for album in self.albums.iter() {
            let downloader = Arc::clone(&self.downloader);
            let album = PlaylistBrowse::new(BrowseId::ChannelBrowseId(album.clone()), downloader, Uuid::new_v4()).browse();
            tasks.push(tokio::spawn(album));
        }

        let mut browsed = Vec::with_capacity(tasks.len());

        for task in tasks {
            browsed.push(task.await??);
        }

        Ok(browsed)
    }

    pub async fn browse_albums(&self) -> Result<Vec<Playlist>> {
        let mut tasks = Vec::new();

        let half = self.half_browse_albums().await?;

        for album in half {
            tasks.push(tokio::spawn(album.browse()));
        }

        let mut browsed = Vec::with_capacity(tasks.len());

        for task in tasks {
            browsed.push(task.await??);
        }

        Ok(browsed)
    }

    pub async fn browse(self) -> Result<Channel> {
        let albums = self.browse_albums().await?;
        let eps = self.browse_eps().await?;
        let singles = self.browse_singles().await?;

        Ok(Channel {
            downloader: self.downloader,
            title: self.title,
            albums,
            eps,
            singles,
            id: self.id,
        })
    }
}
