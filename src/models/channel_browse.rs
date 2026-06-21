use std::sync::Arc;

use serde::Deserialize;
use uuid::Uuid;

use crate::{
    Result,
    downloader::{Downloader, channel::ChannelContentBrowse},
    error::YtuwuError,
    id_resolver::{Id, types::ChannelPlaylistId},
    models::response::{Response, Status},
};

/// This is the response for the results of browsed Channels
/// For example: A channel contains an album. This album can not be browsed with the regular
/// BrowseClient. The SlowBrowseClient has to browse and this is the response.
/// This basic structure allows the extraction if the ids and the title
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelBrowseResponse {
    contents: Option<ChannelContents>,
    header: Option<Header>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Header {
    music_header_renderer: MusicHeaderRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MusicHeaderRenderer {
    title: HeaderTitle,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct HeaderTitle {
    runs: Vec<Title>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Title {
    text: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ChannelContents {
    single_column_browse_results_renderer: SingleColumnBrowseResultsRenderer,
}

#[derive(Deserialize, Debug)]
struct SingleColumnBrowseResultsRenderer {
    tabs: Vec<ChannelTab>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ChannelTab {
    tab_renderer: ChannelTabRenderer,
}

#[derive(Deserialize, Debug)]
struct ChannelTabRenderer {
    content: ChannelTabContent,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ChannelTabContent {
    section_list_renderer: ChannelSectionListRenderer,
}

#[derive(Deserialize, Debug)]
struct ChannelSectionListRenderer {
    contents: Option<Vec<ChannelSection>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ChannelSection {
    grid_renderer: GridRenderer,
}

#[derive(Deserialize, Debug)]
struct GridRenderer {
    items: Vec<GridItem>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GridItem {
    music_two_row_item_renderer: MusicTwoRowItemRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MusicTwoRowItemRenderer {
    navigation_endpoint: ReleaseNavigationEndpoint,
    subtitle: Option<Subtitle>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ReleaseNavigationEndpoint {
    browse_endpoint: ReleaseBrowseEndpoint,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ReleaseBrowseEndpoint {
    browse_id: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Subtitle {
    runs: Vec<SubtitleRun>,
}

#[derive(Deserialize, Debug)]
struct SubtitleRun {
    text: Option<String>,
}

impl ChannelBrowseResponse {
    /// Extracts all singles, eps, albums and the title
    /// Returns a ChannelContentBrowse with all the fields needed
    /// singles, albums and eps are all from type ChannelPlaylistId because youtube thinks that its
    /// a good idea to store singles as a playlist
    pub fn extract_all_releases(self, downloader: Arc<Downloader>, id: Uuid) -> Result<ChannelContentBrowse> {
        let mut albums: Vec<ChannelPlaylistId> = Vec::new();
        let mut eps: Vec<ChannelPlaylistId> = Vec::new();
        let mut singles: Vec<ChannelPlaylistId> = Vec::new();

        let contents = &self
            .contents
            .as_ref()
            .ok_or(YtuwuError::BrowseDataNotFound("contents"))?;

        let tab = &contents
            .single_column_browse_results_renderer
            .tabs
            .get(0)
            .ok_or(YtuwuError::BrowseDataNotFound("tab"))?;

        let section_list = &tab
            .tab_renderer
            .content
            .section_list_renderer;

        let section = &section_list
            .contents
            .as_ref()
            .ok_or(YtuwuError::BrowseDataNotFound("contents"))?
            .get(0)
            .ok_or(YtuwuError::BrowseDataNotFound("contents is empty"))?;

        for item in &section.grid_renderer.items {
            let renderer = &item.music_two_row_item_renderer;

            let browse_id: &str = match &renderer
                .navigation_endpoint
                .browse_endpoint
                .browse_id
            {
                Some(id) => &id,
                None => continue,
            };

            let playlist_id = ChannelPlaylistId::new(browse_id)?;

            let type_text = renderer
                .subtitle
                .as_ref()
                .and_then(|s| s.runs.first())
                .and_then(|r| r.text.as_deref())
                .unwrap_or("")
                .to_lowercase();

            match type_text.as_str() {
                "album" => albums.push(playlist_id),
                "ep" => eps.push(playlist_id),
                "single" => singles.push(playlist_id),
                _ => {}
            }
        }

        let title = self
            .header
            .as_ref()
            .ok_or(YtuwuError::ChannelDataNotFound("header"))?
            .music_header_renderer
            .title
            .runs
            .first()
            .as_ref()
            .ok_or(YtuwuError::ChannelDataNotFound("title"))?
            .text
            .as_ref()
            .ok_or(YtuwuError::ChannelDataNotFound("title"))?;

        Ok(ChannelContentBrowse {
            albums,
            eps,
            singles,
            downloader,
            id,
            title: title.to_owned(),
        })
    }
}

impl Response for ChannelBrowseResponse {
    fn get_status(&self) -> Status {
        if self.contents.is_none() {
            return Status::Error;
        }
        Status::Success
    }
}
