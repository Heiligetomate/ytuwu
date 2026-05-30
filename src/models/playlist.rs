use serde::Deserialize;

use crate::{
    Id,
    error::YtuwuError,
    models::response::{BrowseResponse, Response, Status},
    types::VideoId,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistResponse {
    contents: Option<PlaylistContents>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PlaylistContents {
    two_column_browse_results_renderer: TwoColumnBrowseResultsRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TwoColumnBrowseResultsRenderer {
    secondary_contents: SecondaryContents,
    tabs: Option<Vec<NameTab>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SecondaryContents {
    section_list_renderer: SectionListRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SectionListRenderer {
    contents: Vec<SectionListItem>, // first one
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SectionListItem {
    music_playlist_shelf_renderer: MusicPlaylistShelfRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MusicPlaylistShelfRenderer {
    contents: Vec<MainItem>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MainItem {
    music_responsive_list_item_renderer: Option<MusicResponsiveListItemRenderer>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MusicResponsiveListItemRenderer {
    flex_columns: Vec<FlexColumn>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FlexColumn {
    music_responsive_list_item_flex_column_renderer: MusicResponsiveListItemFlexColumnRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MusicResponsiveListItemFlexColumnRenderer {
    text: Text,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Text {
    runs: Vec<Run>, // 1
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Run {
    navigation_endpoint: Option<NavigationEndpoint>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct NavigationEndpoint {
    watch_endpoint: Option<WatchEndpoint>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct WatchEndpoint {
    video_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct NameTab {
    tab_renderer: TabRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TabRenderer {
    content: PlaylistTitleContent,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PlaylistTitleContent {
    section_list_renderer: SectionListTitleRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SectionListTitleRenderer {
    contents: Vec<NameContents>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct NameContents {
    music_responsive_header_renderer: MusicResponsiveHeaderRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MusicResponsiveHeaderRenderer {
    title: PlaylistRunTitle,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PlaylistRunTitle {
    runs: Vec<PlaylistTitle>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PlaylistTitle {
    text: String,
}

impl Response for PlaylistResponse {
    fn get_status(&self) -> Status {
        match self.contents {
            Some(_) => Status::Success,
            None => Status::Error,
        }
    }
}

impl BrowseResponse for PlaylistResponse {
    fn get_video_ids(&self) -> crate::Result<Vec<crate::types::VideoId>> {
        let ids: Vec<VideoId> = self
            .contents
            .as_ref()
            .ok_or(YtuwuError::BrowseDataNotFound("contents"))?
            .two_column_browse_results_renderer
            .secondary_contents
            .section_list_renderer
            .contents
            .get(0)
            .ok_or(YtuwuError::BrowseDataNotFound("first content"))?
            .music_playlist_shelf_renderer
            .contents
            .iter()
            .filter_map(|m| {
                m.music_responsive_list_item_renderer
                    .as_ref()
            })
            .filter_map(|m| m.flex_columns.get(0))
            .filter_map(|f| {
                f.music_responsive_list_item_flex_column_renderer
                    .text
                    .runs
                    .get(0)
            })
            .filter_map(|r| r.navigation_endpoint.as_ref())
            .filter_map(|n| n.watch_endpoint.as_ref())
            .filter_map(|w| {
                let raw: &str = w.video_id.as_ref();
                VideoId::new(raw).ok()
            })
            .collect();

        Ok(ids)
    }

    fn get_album_title(&self) -> crate::Result<&str> {
        let title = &self
            .contents
            .as_ref()
            .ok_or(YtuwuError::BrowseDataNotFound("contents"))?
            .two_column_browse_results_renderer
            .tabs
            .as_ref()
            .ok_or(YtuwuError::BrowseDataNotFound("title tabs"))?
            .get(0)
            .ok_or(YtuwuError::BrowseDataNotFound("first title tab"))?
            .tab_renderer
            .content
            .section_list_renderer
            .contents
            .get(0)
            .ok_or(YtuwuError::BrowseDataNotFound("name content"))?
            .music_responsive_header_renderer
            .title
            .runs
            .get(0)
            .ok_or(YtuwuError::BrowseDataNotFound("first title tab run"))?
            .text
            .as_ref();

        Ok(title)
    }
}
