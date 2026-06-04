use std::{fmt::Debug, sync::Arc};

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
    browse_id: BrowseId,
    downloader: Arc<Downloader>,
}

impl PlaylistBrowse {
    pub fn new(id: BrowseId, downloader: Arc<Downloader>) -> Self {
        Self { browse_id: id, downloader }
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

        let trimmed_title = name_trimmer::trim(title, "-");

        let media: Vec<MediaBrowse> = ids
            .drain(..)
            .map(|id| MediaBrowse::new(id))
            .collect();
        Ok(PlaylistContentBrowse::new(&trimmed_title, media, self.downloader))
    }
}
