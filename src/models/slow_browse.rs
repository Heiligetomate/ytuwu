use serde::Deserialize;

use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{Id, types::VideoId},
    models::response::{BrowseResponse, Response, Status},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SlowBrowseResponse {
    contents: Option<AlbumContents>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AlbumContents {
    two_column_browse_results_renderer: TwoColumnBrowseResultsRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TwoColumnBrowseResultsRenderer {
    secondary_contents: AlbumSecondaryContents,
    tabs: Vec<Tab>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Tab {
    tab_renderer: TabRenderer,
}

#[derive(Deserialize, Debug)]
struct TabRenderer {
    content: TabRendererContent,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TabRendererContent {
    section_list_renderer: TitleSectionListRenderer,
}

#[derive(Deserialize, Debug)]
struct TitleSectionListRenderer {
    contents: Vec<TitleContents>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TitleContents {
    music_responsive_header_renderer: MusicResponsiveHeaderRenderer,
}

#[derive(Deserialize, Debug)]
struct MusicResponsiveHeaderRenderer {
    title: AlbumTitle,
}

#[derive(Deserialize, Debug)]
struct AlbumTitle {
    runs: Vec<AlbumTitleRun>,
}

#[derive(Deserialize, Debug)]
struct AlbumTitleRun {
    text: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AlbumSecondaryContents {
    section_list_renderer: AlbumSectionListRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AlbumSectionListRenderer {
    contents: Vec<AlbumSection>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AlbumSection {
    music_shelf_renderer: Option<MusicShelfRenderer>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MusicShelfRenderer {
    contents: Vec<AlbumTrackItem>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AlbumTrackItem {
    music_responsive_list_item_renderer: MusicResponsiveListItemRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MusicResponsiveListItemRenderer {
    navigation_endpoint: Option<NavigationEndpoint>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct NavigationEndpoint {
    watch_endpoint: WatchEndpoint,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct WatchEndpoint {
    video_id: String,
}

impl Response for SlowBrowseResponse {
    fn get_status(&self) -> Status {
        match self.contents {
            Some(_) => Status::Success,
            None => Status::Error,
        }
    }
}

impl BrowseResponse for SlowBrowseResponse {
    fn get_video_ids(&self) -> Result<Vec<VideoId>> {
        let ids = self
            .contents
            .as_ref()
            .ok_or(YtuwuError::BrowseDataNotFound("contents"))?
            .two_column_browse_results_renderer
            .secondary_contents
            .section_list_renderer
            .contents
            .iter() // i thibj thgat zsgozd just be tge furst nbe
            .filter_map(|section| section.music_shelf_renderer.as_ref())
            .flat_map(|shelf| shelf.contents.iter())
            .filter_map(|item| {
                item.music_responsive_list_item_renderer
                    .navigation_endpoint
                    .as_ref()
            })
            .filter_map(|n| {
                let id = n.watch_endpoint.video_id.as_str();
                VideoId::new(id).ok()
            })
            .collect();

        Ok(ids)
    }

    fn get_album_title(&self) -> Result<&str> {
        let title = &self
            .contents
            .as_ref()
            .ok_or(YtuwuError::BrowseDataNotFound("contents"))?
            .two_column_browse_results_renderer
            .tabs
            .get(0)
            .ok_or(YtuwuError::BrowseDataNotFound("first content"))?
            .tab_renderer
            .content
            .section_list_renderer
            .contents
            .get(0)
            .ok_or(YtuwuError::BrowseDataNotFound("first content"))?
            .music_responsive_header_renderer
            .title
            .runs
            .get(0)
            .ok_or(YtuwuError::BrowseDataNotFound("first Tab"))?
            .text;

        Ok(title)
    }
}
