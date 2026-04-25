use serde::Deserialize;

use crate::browse_model::playlist_renderer::PlaylistVideoListRenderer;
use crate::error::{Result, YtuwuError};

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
#[serde(rename_all = "camelCase")]
struct TabRenderer {
    content: TabRendererContent,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TabRendererContent {
    section_list_renderer: SectionListRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SectionListRenderer {
    contents: Option<Vec<ListRendererContent>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ListRendererContent {
    playlist_video_list_renderer: Option<PlaylistVideoListRenderer>,
    //shelf_renderer              : Option<ShelfRenderer>,
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
            .ok_or(YtuwuError::BrowseDataNotFound(
                "section list renderer contents",
            ))?
            .get(0)
            .as_ref()
            .ok_or(YtuwuError::BrowseDataNotFound(
                "first section list renderer element",
            ))?
            .playlist_video_list_renderer
            .as_ref()
            .ok_or(YtuwuError::BrowseDataNotFound(
                "playlist video list renderer",
            ))?
            .get_ids();
        Ok(ids)
    }
}
