nya = data;

const items = nya.contents.singleColumnBrowseResultsRenderer
  .tabs[0].tabRenderer.content.sectionListRenderer.contents[0]
  .gridRenderer.items;

const albumIds = items
  .map(item => item.musicTwoRowItemRenderer?.navigationEndpoint.browseEndpoint.browseId)
  .filter(Boolean);

console.log(albumIds);
