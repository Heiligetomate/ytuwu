use std::{fmt::Debug, sync::Arc};

use uuid::Uuid;

use crate::{
    Result,
    downloader::{Downloader, media::MediaBrowse, playlist::content_browse::PlaylistContentBrowse},
    models::response::BrowseResponse,
    name_trimmer,
    request::api_request,
    types::BrowseId,
};

/// This struct is used for browsing playlists and extracting all video ids from a playlist
/// It holds the browseid which can be any of the variants that browseid holds and an arc of the
/// downloader to use shared data and an uuid for identification
#[derive(Debug)]
pub struct PlaylistBrowse {
    id: Uuid,
    browse_id: BrowseId,
    downloader: Arc<Downloader>,
}

impl PlaylistBrowse {
    /// Creates a new instance of itself with the given parameters  
    pub fn new(browse_id: BrowseId, downloader: Arc<Downloader>, id: Uuid) -> Self {
        Self { browse_id, downloader, id }
    }

    // TODO: Cant we use the browseId trait here?
    /// Matches against self and makes an api request with the browse id
    /// After that the ids and the title get extracted
    /// The title gets trimmed and the ids get merged into a vec of MediaBrowses
    /// After that a new instance of PlaylistContentBrowse gets created with the collected data
    pub async fn browse(self) -> Result<PlaylistContentBrowse> {
        let (mut ids, title) = match self.browse_id {
            BrowseId::AlbumId(id) => {
                let response = api_request(&id, &self.downloader.client).await?;
                let ids = response.get_video_ids()?;
                let title = response.get_album_title()?.to_owned();
                (ids, title)
            }
            BrowseId::PlaylistId(id) => {
                let response = api_request(&id, &self.downloader.client).await?;
                let ids = response.get_video_ids()?;
                let title = response.get_album_title()?.to_owned();
                (ids, title)
            }

            BrowseId::ChannelBrowseId(id) => {
                let response = api_request(&id, &self.downloader.client).await?;
                let ids = response.get_video_ids()?;
                let title = response.get_album_title()?.to_owned();
                (ids, title)
            }
        };

        let trimmed_title = name_trimmer::default_trim(&title);

        let media: Vec<MediaBrowse> = ids
            .drain(..)
            .map(|id| MediaBrowse::new(id, Uuid::new_v4()))
            .collect();
        Ok(PlaylistContentBrowse::new(&trimmed_title, media, self.downloader, self.id))
    }
}
