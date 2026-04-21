use anyhow::{Result, anyhow};
use serde::Deserialize;

use crate::browse_model::playlist_renderer::PlaylistVideoListRenderer;

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
            .ok_or(anyhow!("first element of tabs doesnt exist"))?
            .tab_renderer
            .as_ref()
            .ok_or(anyhow!("no tabrenderer found"))?
            .content
            .section_list_renderer
            .contents
            .as_ref()
            .ok_or(anyhow!("no section list renderer contents found"))?
            .get(0)
            .as_ref()
            .ok_or(anyhow!("first element of section list renderer doesnt exit"))?
            .playlist_video_list_renderer
            .as_ref()
            .ok_or(anyhow!("no shelf renderer found"))?
            .get_ids(); 
        Ok(ids)    
    }
}
