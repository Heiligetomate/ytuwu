use serde::Deserialize;

use crate::{
    error::{Result, YtuwuError},
    id_resolver::{Id, types::VideoId},
    models::response::{BrowseResponse, Response, Status},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FastBrowseResponse {
    error: Option<ErrorResponse>,
    contents: Option<FullResponse>,
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

impl BrowseResponse for FastBrowseResponse {
    fn get_album_title(&self) -> Result<&str> {
        let title = &self
            .header
            .as_ref()
            .ok_or(YtuwuError::BrowseDataNotFound("header"))?
            .playlist_header_renderer
            .title
            .runs
            .get(0)
            .ok_or(YtuwuError::BrowseDataNotFound("album title"))?
            .text
            .as_str();
        Ok(title)
    }

    fn get_video_ids(&self) -> Result<Vec<VideoId>> {
        let ids = self
            .contents
            .as_ref()
            .ok_or(YtuwuError::BrowseDataNotFound("contents"))?
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
            .contents
            .iter()
            .filter_map(|item| {
                item.playlist_video_renderer
                    .video_id
                    .as_ref()
            })
            .filter_map(|id| VideoId::new(id.as_str()).ok())
            .collect();

        Ok(ids)
    }
}

impl Response for FastBrowseResponse {
    fn get_status(&self) -> Status {
        if self.error.is_some() {
            return Status::Error;
        }
        Status::Success
    }
}
