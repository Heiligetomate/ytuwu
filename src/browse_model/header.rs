use anyhow::{Result, anyhow};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BrowseHeader {
   playlist_header_renderer: HeaderRenderer, 
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct HeaderRenderer {
    title: HeaderTitle,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct HeaderTitle {
    runs: Vec<AlbumTitle>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AlbumTitle {
    text: String,
}

impl BrowseHeader {
    pub fn get_album_title(&self) -> Result<&str> {
        let title_object = &self.playlist_header_renderer.title;
        let title = title_object.runs.get(0).ok_or(anyhow!("title was not found"))?.text.as_str();
        Ok(title)
    }
}
