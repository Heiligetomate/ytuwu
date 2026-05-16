use serde::Deserialize;

use crate::models::response::Response;

#[derive(Deserialize, Debug)]
pub struct SlowAlbumResponse {
    contents: AlbumContents,
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
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AlbumSecondaryContents {
    section_list_renderer: AlbumSectionListRenderer,
}

#[derive(Deserialize, Debug)]
struct AlbumSectionListRenderer {
    contents: Vec<AlbumSection>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AlbumSection {
    music_playlist_shelf_renderer: MusicPlaylistShelfRenderer,
}

#[derive(Deserialize, Debug)]
struct MusicPlaylistShelfRenderer {
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
    playlist_item_data: PlaylistItemData,
    flex_columns: Vec<FlexColumn>,
    thumbnail: TrackThumbnail,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PlaylistItemData {
    video_id: String,
    playlist_set_video_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FlexColumn {
    music_responsive_list_item_flex_column_renderer: FlexColumnRenderer,
}

#[derive(Deserialize, Debug)]
struct FlexColumnRenderer {
    text: FlexColumnText,
}

#[derive(Deserialize, Debug)]
struct FlexColumnText {
    runs: Vec<FlexColumnRun>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FlexColumnRun {
    text: String,
    navigation_endpoint: Option<WatchEndpointWrapper>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct WatchEndpointWrapper {
    watch_endpoint: Option<WatchEndpoint>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct WatchEndpoint {
    video_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TrackThumbnail {
    music_thumbnail_renderer: MusicThumbnailRenderer,
}

#[derive(Deserialize, Debug)]
struct MusicThumbnailRenderer {
    thumbnail: ThumbnailList,
}

#[derive(Deserialize, Debug)]
struct ThumbnailList {
    thumbnails: Vec<Thumbnail>,
}

#[derive(Deserialize, Debug)]
struct Thumbnail {
    url: String,
    width: u32,
    height: u32,
}

impl Response for SlowAlbumResponse {
    fn get_status(&self) -> super::response::Status {
        todo!()
    }

    fn get_visitor_data(&self) -> Option<String> {
        todo!()
    }
}
