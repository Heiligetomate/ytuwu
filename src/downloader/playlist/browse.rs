use std::{fmt::Debug, sync::Arc};

use uuid::Uuid;

use crate::{
    Downloader, Result,
    downloader::{media::browse::MediaBrowse, playlist::content_browse::PlaylistContentBrowse},
    models::response::BrowseResponse,
    name_trimmer,
    request::core::api_request,
    types::BrowseId,
};

#[derive(Debug)]
pub struct PlaylistBrowse {
    id: Uuid,
    browse_id: BrowseId,
    downloader: Arc<Downloader>,
}

impl PlaylistBrowse {
    pub fn new(browse_id: BrowseId, downloader: Arc<Downloader>, id: Uuid) -> Self {
        Self { browse_id, downloader, id }
    }

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
