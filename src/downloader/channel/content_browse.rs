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

// TODO: There should be an additional step i think (half browse looks wrong here)

/// This struct gets created after a channel was browsed and contains a vec of singles, eps and
/// albums where each of those contain a ChannelPlaylistId which has to be converted or browsed
/// afterwards.
/// This struct also contains the title of the channel, an arc downloader for shared data and an
/// uuid for identification.
/// This is an in-between step for downloading a channel which is needed because the channel brose
/// just returns a bunch of playlist ids that then have to be browsed again.
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
    /// Browses all singles, eps and albums so that there are all video ids available
    /// Collects all ids from those objects and creates tasks for the task handler
    /// Spawns all of these tasks with the vorrect id which for eps is the playlist id and for all
    /// of them the normal already existing id and the channel id contained in self
    /// This is important if the downloaded medias want to be extracted later correctly
    /// The metadata is created from the already existing metadata which contains only a title.
    /// A Template is created that contains all ep, single and album ids. This template then gets
    /// pushed on the downloader shared storage with the own channel id as key.
    /// Fails if the browsing of the eps, albums or singles failes.
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

    /// Browses all singles, eps and albums so that there are all video ids available
    /// Collects all ids from those objects and creates tasks for the task handler
    /// Spawns all of these tasks as bundle tasks with the correct id which for eps and albums
    /// is the playlist id and for all of them the normal already existing id and the channel id contained in self
    /// This is important if the downloaded medias want to be extracted later correctly
    /// The metadata is created from the already existing metadata which contains only a title.
    /// A Template is created that contains all ep, single and album ids. This template then gets
    /// pushed on the downloader shared storage with the own channel id as key.
    /// Fails if the browsing of the eps, albums or singles failes.
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

    /// Creates a tokio task for every single contained in self.singles.
    /// Only browses so far to get the media browses which contain the needed video id by creating a
    /// new playlist browse with the ChannelBrowseId
    /// Because those are singles, the first song is extracted.
    /// Awaits all of those tasks, collects and returns them
    /// Fails if any of the tasks failed to browse.
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

    /// Calls half_browse to get the media browses and creates a task for every extracted single
    /// media browse which is awaited and collected afterwards.
    /// Fails if the half browse failed or if any of the medias failed to browse afterwards.
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

    /// Creates a tokio task for every ep in self.eps by creating a new playlist browse with the ep
    /// browse id and a newly generated uuid.
    /// Awaits all of those tasks afterwards and returns a vec of PlaylistContentBrowse
    /// Fails if any of the browsing failed
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

    /// Calls self.half_browse_eps and full browses the half browsed results completely afterwards
    /// so that a vec of full browsed playlist gets created. This is achieved by spawning multiple
    /// tasks and awaiting them afterwards.
    /// This fails if any of the browse tasks fails.
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

    /// Creates a tokio task for every album in self.albums by creating a new playlist browse with the album
    /// browse id and a newly generated uuid.
    /// Awaits all of those tasks afterwards and returns a vec of PlaylistContentBrowse
    /// Fails if any of the browsing failed
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

    /// Calls self.half_browse_albums and full browses the half browsed results completely afterwards
    /// so that a vec of full browsed playlist gets created. This is achieved by spawning multiple
    /// tasks and awaiting them afterwards.
    /// This fails if any of the browse tasks fails.
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

    /// Consumes itself and returns a new Channel
    /// Browses all albums, all singles and all eps fully so that there is a vec of media and 2 vecs
    /// of playlists
    /// Creates a new Channel instance afterwards with the browsed data, the already existing title,
    /// id and downloader.
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
