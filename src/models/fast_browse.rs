use serde::Deserialize;

use crate::{
    error::{ResponseDataError, Result, YtuwuError},
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
struct ErrorResponse {}

/// This response is for the regular BrowseClient
/// Its used for browing albums and has the basic structure to get the ids and the title
/// It implements the Response trait and it also implements the BrowseResponse trait
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FullResponse {
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
struct PlaylistVideoListRenderer {
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
            .ok_or(YtuwuError::ResponseData(ResponseDataError::FastBrowse("header")))?
            .playlist_header_renderer
            .title
            .runs
            .get(0)
            .ok_or(YtuwuError::ResponseData(ResponseDataError::FastBrowse("album title ")))?
            .text
            .as_str();
        Ok(title)
    }

    fn get_video_ids(&self) -> Result<Vec<VideoId>> {
        let ids = self
            .contents
            .as_ref()
            .ok_or(YtuwuError::ResponseData(ResponseDataError::FastBrowse("contents")))?
            .single_column_browse_results_renderer
            .tabs
            .get(0)
            .as_ref()
            .ok_or(YtuwuError::ResponseData(ResponseDataError::FastBrowse("tabs")))?
            .tab_renderer
            .as_ref()
            .ok_or(YtuwuError::ResponseData(ResponseDataError::FastBrowse("tab renderer")))?
            .content
            .section_list_renderer
            .contents
            .as_ref()
            .ok_or(YtuwuError::ResponseData(ResponseDataError::FastBrowse("section list renderer contents")))?
            .get(0)
            .as_ref()
            .ok_or(YtuwuError::ResponseData(ResponseDataError::FastBrowse("first section list renderer element")))?
            .playlist_video_list_renderer
            .as_ref()
            .ok_or(YtuwuError::ResponseData(ResponseDataError::FastBrowse("playlist video list renderer")))?
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
