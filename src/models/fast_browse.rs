use serde::Deserialize;

use crate::{
    error::{Result, YtuwuError},
    models::response::{Response, Status},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BrowseResponse {
    error: Option<ErrorResponse>,
    contents: Option<FullResponse>,
    response_context: Option<ResponseContext>,
    header: Option<BrowseHeader>,
}

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FullResponse {
    single_column_browse_results_renderer: ResultRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ResultRenderer {
    tabs: Vec<Tab>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Tab {
    tab_renderer: Option<TabRenderer>,
}

#[derive(Deserialize, Debug)]
struct TabRenderer {
    content: TabRendererContent,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TabRendererContent {
    section_list_renderer: SectionListRenderer,
}

#[derive(Deserialize, Debug)]
struct SectionListRenderer {
    contents: Option<Vec<ListRendererContent>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ListRendererContent {
    playlist_video_list_renderer: Option<PlaylistVideoListRenderer>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BrowseHeader {
    playlist_header_renderer: HeaderRenderer,
}

#[derive(Deserialize, Debug)]
struct HeaderRenderer {
    title: HeaderTitle,
}

#[derive(Deserialize, Debug)]
struct HeaderTitle {
    runs: Vec<AlbumTitle>,
}

#[derive(Deserialize, Debug)]
struct AlbumTitle {
    text: String,
}

#[derive(Deserialize, Debug)]
pub struct PlaylistVideoListRenderer {
    contents: Vec<PlaylistContent>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PlaylistContent {
    playlist_video_renderer: PlaylistVideoRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PlaylistVideoRenderer {
    video_id: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseContext {
    pub visitor_data: Option<String>,
}

impl PlaylistVideoListRenderer {
    pub fn get_ids(&self) -> Vec<&str> {
        let items = &self.contents;
        let ids = items
            .iter()
            .filter_map(|item| {
                item.playlist_video_renderer
                    .video_id
                    .as_ref()
            })
            .map(|id| id.as_str())
            .collect();
        ids
    }
}

impl BrowseHeader {
    pub fn get_album_title(&self) -> Result<&str> {
        let title_object = &self.playlist_header_renderer.title;
        let title = title_object
            .runs
            .get(0)
            .ok_or(YtuwuError::BrowseDataNotFound("album title"))?
            .text
            .as_str();
        Ok(title)
    }
}

impl FullResponse {
    pub fn get_ids(&self) -> Result<Vec<&str>> {
        let ids = self
            .single_column_browse_results_renderer
            .tabs
            .get(0)
            .as_ref()
            .ok_or(YtuwuError::BrowseDataNotFound("tabs"))?
            .tab_renderer
            .as_ref()
            .ok_or(YtuwuError::BrowseDataNotFound("tab renderer"))?
            .content
            .section_list_renderer
            .contents
            .as_ref()
            .ok_or(YtuwuError::BrowseDataNotFound("section list renderer contents"))?
            .get(0)
            .as_ref()
            .ok_or(YtuwuError::BrowseDataNotFound("first section list renderer element"))?
            .playlist_video_list_renderer
            .as_ref()
            .ok_or(YtuwuError::BrowseDataNotFound("playlist video list renderer"))?
            .get_ids();
        Ok(ids)
    }
}

impl BrowseResponse {
    pub fn get_ids(&self) -> Result<Vec<&str>> {
        let ids = self
            .contents
            .as_ref()
            .ok_or(YtuwuError::BrowseDataNotFound("video ids"))?
            .get_ids()?;

        Ok(ids)
    }

    pub fn get_album_title(&self) -> Result<&str> {
        let header = self
            .header
            .as_ref()
            .ok_or(YtuwuError::BrowseDataNotFound("album title"))?;
        let title = &header.get_album_title()?;
        Ok(title)
    }
}

impl Response for BrowseResponse {
    fn get_status(&self) -> Status {
        if self.error.is_some() {
            return Status::Error;
        }
        Status::Success
    }

    fn get_visitor_data(&self) -> Option<String> {
        if let Some(response_context) = &self.response_context {
            return response_context.visitor_data.clone();
        }
        None
    }
}
