nya = data;

const tracks = nya.contents.twoColumnBrowseResultsRenderer
  .secondaryContents.sectionListRenderer.contents[0]
  .musicPlaylistShelfRenderer.contents;

const videoIds = tracks
  .filter(item => item.musicResponsiveListItemRenderer?.playlistItemData)
  .map(item => item.musicResponsiveListItemRenderer.playlistItemData.videoId);

console.log(videoIds);
