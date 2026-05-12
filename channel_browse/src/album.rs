use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AlbumResponse {
    pub contents: AlbumContents,
}

#[derive(Deserialize, Debug)]
pub struct AlbumContents {
    pub twoColumnBrowseResultsRenderer: TwoColumnBrowseResultsRenderer,
}

#[derive(Deserialize, Debug)]
pub struct TwoColumnBrowseResultsRenderer {
    pub secondaryContents: AlbumSecondaryContents,
}

#[derive(Deserialize, Debug)]
pub struct AlbumSecondaryContents {
    pub sectionListRenderer: AlbumSectionListRenderer,
}

#[derive(Deserialize, Debug)]
pub struct AlbumSectionListRenderer {
    pub contents: Vec<AlbumSection>,
}

#[derive(Deserialize, Debug)]
pub struct AlbumSection {
    pub musicPlaylistShelfRenderer: MusicPlaylistShelfRenderer,
}

#[derive(Deserialize, Debug)]
pub struct MusicPlaylistShelfRenderer {
    pub contents: Vec<AlbumTrackItem>,
}

#[derive(Deserialize, Debug)]
pub struct AlbumTrackItem {
    pub musicResponsiveListItemRenderer: MusicResponsiveListItemRenderer,
}

#[derive(Deserialize, Debug)]
pub struct MusicResponsiveListItemRenderer {
    pub playlistItemData: PlaylistItemData,
    pub flexColumns: Vec<FlexColumn>,
    pub thumbnail: TrackThumbnail,
}

#[derive(Deserialize, Debug)]
pub struct PlaylistItemData {
    pub videoId: String,
    pub playlistSetVideoId: String, // can ignore
}

#[derive(Deserialize, Debug)]
pub struct FlexColumn {
    pub musicResponsiveListItemFlexColumnRenderer: FlexColumnRenderer,
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
pub struct FlexColumnRun {
    pub text: String, // col[0] = track title, col[1] = artist, col[2] = duration
    pub navigationEndpoint: Option<WatchEndpointWrapper>,
}

#[derive(Deserialize, Debug)]
pub struct WatchEndpointWrapper {
    pub watchEndpoint: Option<WatchEndpoint>,
}

#[derive(Deserialize, Debug)]
pub struct WatchEndpoint {
    pub videoId: String,
}

#[derive(Deserialize, Debug)]
pub struct TrackThumbnail {
    pub musicThumbnailRenderer: MusicThumbnailRenderer,
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
