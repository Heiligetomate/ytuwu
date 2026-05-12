use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SlowAlbumResponse {
    pub contents: AlbumContents,
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
pub struct AlbumSectionListRenderer {
    pub contents: Vec<AlbumSection>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlbumSection {
    pub music_playlist_shelf_renderer: MusicPlaylistShelfRenderer,
}

#[derive(Deserialize, Debug)]
pub struct MusicPlaylistShelfRenderer {
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
    pub thumbnail: TrackThumbnail,
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
pub struct FlexColumnRenderer {
    pub text: FlexColumnText,
}

#[derive(Deserialize, Debug)]
pub struct FlexColumnText {
    pub runs: Vec<FlexColumnRun>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FlexColumnRun {
    pub text: String,
    pub navigation_endpoint: Option<WatchEndpointWrapper>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WatchEndpointWrapper {
    pub watch_endpoint: Option<WatchEndpoint>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WatchEndpoint {
    pub video_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrackThumbnail {
    pub music_thumbnail_renderer: MusicThumbnailRenderer,
}

#[derive(Deserialize, Debug)]
pub struct MusicThumbnailRenderer {
    pub thumbnail: ThumbnailList,
}

#[derive(Deserialize, Debug)]
pub struct ThumbnailList {
    pub thumbnails: Vec<Thumbnail>,
}

#[derive(Deserialize, Debug)]
pub struct Thumbnail {
    pub url: String,
    pub width: u32,
    pub height: u32,
}
