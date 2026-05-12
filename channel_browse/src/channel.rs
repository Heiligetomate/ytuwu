use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ChannelBrowse {
    pub contents: ChannelContents,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelContents {
    pub single_column_browse_results_renderer: SingleColumnBrowseResultsRenderer,
}

#[derive(Deserialize, Debug)]
pub struct SingleColumnBrowseResultsRenderer {
    pub tabs: Vec<ChannelTab>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelTab {
    pub tab_renderer: ChannelTabRenderer,
}

#[derive(Deserialize, Debug)]
pub struct ChannelTabRenderer {
    pub content: ChannelTabContent,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelTabContent {
    pub section_list_renderer: ChannelSectionListRenderer,
}

#[derive(Deserialize, Debug)]
pub struct ChannelSectionListRenderer {
    pub contents: Vec<ChannelSection>,
    pub header: Option<ChannelSectionListHeader>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSectionListHeader {
    pub music_side_aligned_item_renderer: MusicSideAlignedItemRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MusicSideAlignedItemRenderer {
    pub start_items: Vec<ChipCloud>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChipCloud {
    pub chip_cloud_renderer: ChipCloudRenderer,
}

#[derive(Deserialize, Debug)]
pub struct ChipCloudRenderer {
    pub chips: Vec<Chip>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Chip {
    pub chip_cloud_chip_renderer: ChipCloudChipRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChipCloudChipRenderer {
    pub text: ChipText,
    pub navigation_endpoint: ChipNavigationEndpoint,
}

#[derive(Deserialize, Debug)]
pub struct ChipText {
    pub runs: Vec<ChipRun>,
}

#[derive(Deserialize, Debug)]
pub struct ChipRun {
    pub text: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChipNavigationEndpoint {
    pub browse_section_list_reload_endpoint: BrowseSectionListReloadEndpoint,
}

#[derive(Deserialize, Debug)]
pub struct BrowseSectionListReloadEndpoint {
    pub continuation: ContinuationWrapper,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContinuationWrapper {
    pub reload_continuation_data: ReloadContinuationData,
}

#[derive(Deserialize, Debug)]
pub struct ReloadContinuationData {
    pub continuation: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSection {
    pub grid_renderer: GridRenderer,
}

#[derive(Deserialize, Debug)]
pub struct GridRenderer {
    pub items: Vec<GridItem>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GridItem {
    pub music_two_row_item_renderer: MusicTwoRowItemRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MusicTwoRowItemRenderer {
    pub navigation_endpoint: ReleaseNavigationEndpoint,
    pub subtitle: Option<Subtitle>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseNavigationEndpoint {
    pub browse_endpoint: ReleaseBrowseEndpoint,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseBrowseEndpoint {
    pub browse_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Subtitle {
    pub runs: Vec<SubtitleRun>,
}

#[derive(Deserialize, Debug)]
pub struct SubtitleRun {
    pub text: String,
}
