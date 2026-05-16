use serde::Deserialize;

use crate::{
    Result,
    error::YtuwuError,
    id_resolver::{id::Id, video_id::VideoId},
    models::response::{BrowseResponse, Response, Status},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SlowAlbumResponse {
    pub contents: Option<AlbumContents>,
    pub response_context: Option<ResponseContext>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseContext {
    pub visitor_data: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlbumContents {
    pub two_column_browse_results_renderer: TwoColumnBrowseResultsRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TwoColumnBrowseResultsRenderer {
    pub secondary_contents: AlbumSecondaryContents,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlbumSecondaryContents {
    pub section_list_renderer: AlbumSectionListRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlbumSectionListRenderer {
    pub contents: Vec<AlbumSection>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlbumSection {
    pub music_shelf_renderer: Option<MusicShelfRenderer>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MusicShelfRenderer {
    pub contents: Vec<AlbumTrackItem>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlbumTrackItem {
    pub music_responsive_list_item_renderer: MusicResponsiveListItemRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MusicResponsiveListItemRenderer {
    pub playlist_item_data: PlaylistItemData,
    pub flex_columns: Vec<FlexColumn>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemData {
    pub video_id: String,
    pub playlist_set_video_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FlexColumn {
    pub music_responsive_list_item_flex_column_renderer: FlexColumnRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FlexColumnRenderer {
    pub text: FlexColumnText,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FlexColumnText {
    pub runs: Vec<FlexColumnRun>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FlexColumnRun {
    pub text: String,
}

impl Response for SlowAlbumResponse {
    fn get_status(&self) -> Status {
        match self.contents {
            Some(_) => Status::Success,
            None => match self.response_context {
                Some(_) => Status::Login,
                None => Status::Error,
            },
        }
    }

    fn get_visitor_data(&self) -> Option<String> {
        if let Some(ctx) = &self.response_context {
            return ctx.visitor_data.clone();
        }
        None
    }
}

impl BrowseResponse for SlowAlbumResponse {
    fn get_video_ids(&self) -> Result<Vec<VideoId>> {
        let ids = self
            .contents
            .as_ref()
            .ok_or(YtuwuError::BrowseDataNotFound("contents"))?
            .two_column_browse_results_renderer
            .secondary_contents
            .section_list_renderer
            .contents
            .iter()
            .filter_map(|section| section.music_shelf_renderer.as_ref())
            .flat_map(|shelf| shelf.contents.iter())
            .map(|item| {
                let id = item
                    .music_responsive_list_item_renderer
                    .playlist_item_data
                    .video_id
                    .as_str();
                VideoId::new(id)
            })
            .collect();

        Ok(ids)
    }
}
