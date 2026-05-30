use std::fmt::Debug;

use serde::de::DeserializeOwned;

use crate::{
    Result,
    downloader::{media::browse::MediaBrowse, playlist::content_browse::PlaylistContentBrowse},
    id_resolver::{browse_id::BrowseId, id::Id},
    models::response::BrowseResponse,
    name_trimmer,
    request::{clients::client::ClientWithHeaders, core::api_request},
};

#[derive(Debug)]
pub struct PlaylistBrowse<B: BrowseId> {
    browse_id: B,
}

impl<B: BrowseId> PlaylistBrowse<B>
where
    <<B as Id>::Client as ClientWithHeaders>::Response: DeserializeOwned + Debug,
    <<B as Id>::Client as ClientWithHeaders>::Response: BrowseResponse,
{
    pub fn new(id: B) -> Self {
        Self { browse_id: id }
    }
    pub async fn browse(self) -> Result<PlaylistContentBrowse> {
        let response = api_request(&self.browse_id).await?;
        let mut ids = response.get_video_ids()?;
        let title = response.get_album_title()?.to_owned();
        let trimmed_title = name_trimmer::trim(title, "-");
        let media: Vec<MediaBrowse> = ids
            .drain(..)
            .map(|id| MediaBrowse::new(id))
            .collect();
        Ok(PlaylistContentBrowse::new(&trimmed_title, media))
    }
}
