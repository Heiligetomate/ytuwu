use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ChannelBrowse {
    contents: ChannelContents,
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
    contents: Vec<ChannelSection>,
    header: Option<ChannelSectionListHeader>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ChannelSectionListHeader {
    music_side_aligned_item_renderer: MusicSideAlignedItemRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MusicSideAlignedItemRenderer {
    start_items: Vec<ChipCloud>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ChipCloud {
    chip_cloud_renderer: ChipCloudRenderer,
}

#[derive(Deserialize, Debug)]
struct ChipCloudRenderer {
    chips: Vec<Chip>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Chip {
    chip_cloud_chip_renderer: ChipCloudChipRenderer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ChipCloudChipRenderer {
    text: ChipText,
    navigation_endpoint: ChipNavigationEndpoint,
}

#[derive(Deserialize, Debug)]
struct ChipText {
    runs: Vec<ChipRun>,
}

#[derive(Deserialize, Debug)]
struct ChipRun {
    text: String, // "Albums" or "Singles & EPs"
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ChipNavigationEndpoint {
    browse_section_list_reload_endpoint: BrowseSectionListReloadEndpoint,
}

#[derive(Deserialize, Debug)]
struct BrowseSectionListReloadEndpoint {
    continuation: ContinuationWrapper,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ContinuationWrapper {
    reload_continuation_data: ReloadContinuationData,
}

#[derive(Deserialize, Debug)]
struct ReloadContinuationData {
    continuation: String,
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
    browse_id: String,
}

#[derive(Deserialize, Debug)]
struct Subtitle {
    runs: Vec<SubtitleRun>,
}

#[derive(Deserialize, Debug)]
struct SubtitleRun {
    text: String,
}
