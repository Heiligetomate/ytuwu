use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ChannelBrowse {
    contents: ChannelContents,
}

#[derive(Deserialize, Debug)]
struct ChannelContents {
    singleColumnBrowseResultsRenderer: SingleColumnBrowseResultsRenderer,
}

#[derive(Deserialize, Debug)]
struct SingleColumnBrowseResultsRenderer {
    tabs: Vec<ChannelTab>,
}

#[derive(Deserialize, Debug)]
struct ChannelTab {
    tabRenderer: ChannelTabRenderer,
}

#[derive(Deserialize, Debug)]
struct ChannelTabRenderer {
    content: ChannelTabContent,
}

#[derive(Deserialize, Debug)]
struct ChannelTabContent {
    sectionListRenderer: ChannelSectionListRenderer,
}

#[derive(Deserialize, Debug)]
struct ChannelSectionListRenderer {
    contents: Vec<ChannelSection>,
    header: Option<ChannelSectionListHeader>,
}

#[derive(Deserialize, Debug)]
struct ChannelSectionListHeader {
    musicSideAlignedItemRenderer: MusicSideAlignedItemRenderer,
}

#[derive(Deserialize, Debug)]
struct MusicSideAlignedItemRenderer {
    startItems: Vec<ChipCloud>,
}

#[derive(Deserialize, Debug)]
struct ChipCloud {
    chipCloudRenderer: ChipCloudRenderer,
}

#[derive(Deserialize, Debug)]
struct ChipCloudRenderer {
    chips: Vec<Chip>,
}

#[derive(Deserialize, Debug)]
struct Chip {
    chipCloudChipRenderer: ChipCloudChipRenderer,
}

#[derive(Deserialize, Debug)]
struct ChipCloudChipRenderer {
    text: ChipText,
    navigationEndpoint: ChipNavigationEndpoint,
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
struct ChipNavigationEndpoint {
    browseSectionListReloadEndpoint: BrowseSectionListReloadEndpoint,
}

#[derive(Deserialize, Debug)]
struct BrowseSectionListReloadEndpoint {
    continuation: ContinuationWrapper,
}

#[derive(Deserialize, Debug)]
struct ContinuationWrapper {
    reloadContinuationData: ReloadContinuationData,
}

#[derive(Deserialize, Debug)]
struct ReloadContinuationData {
    continuation: String, // pass this as "continuation" to get that tab
}

#[derive(Deserialize, Debug)]
struct ChannelSection {
    gridRenderer: GridRenderer,
}

#[derive(Deserialize, Debug)]
struct GridRenderer {
    items: Vec<GridItem>,
}

#[derive(Deserialize, Debug)]
struct GridItem {
    musicTwoRowItemRenderer: MusicTwoRowItemRenderer,
}

#[derive(Deserialize, Debug)]
struct MusicTwoRowItemRenderer {
    navigationEndpoint: ReleaseNavigationEndpoint,
    subtitle: Option<Subtitle>,
}

#[derive(Deserialize, Debug)]
struct ReleaseNavigationEndpoint {
    browseEndpoint: ReleaseBrowseEndpoint,
}

#[derive(Deserialize, Debug)]
struct ReleaseBrowseEndpoint {
    browseId: String, // MPREb_ ID
}

#[derive(Deserialize, Debug)]
struct Subtitle {
    runs: Vec<SubtitleRun>,
}

#[derive(Deserialize, Debug)]
struct SubtitleRun {
    text: String, // concatenate all to get e.g. "Single • 2024"
}
