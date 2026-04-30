nya = data;

discographyItems = nya.contents.singleColumnBrowseResultsRenderer
  .tabs[0].tabRenderer.content.sectionListRenderer.contents[0]
  .gridRenderer.items;

// Helper to extract browse ID from an item if it's a valid musicTwoRowItemRenderer
function getBrowseId(item) {
  const twoRow = item.musicTwoRowItemRenderer;
  if (!twoRow) return null;
  return twoRow.navigationEndpoint?.browseEndpoint?.browseId || null;
}

// Helper to get the type (Album, Single, EP, etc.)
function getType(item) {
  const twoRow = item.musicTwoRowItemRenderer;
  if (!twoRow) return null;
  const subtitleRuns = twoRow.subtitle?.runs;
  if (!subtitleRuns || subtitleRuns.length === 0) return null;
  return subtitleRuns[0].text; // e.g., "Album", "Single", "EP"
}

const albums = discographyItems
  .filter(item => getType(item) === 'Album')
  .map(item => getBrowseId(item))
  .filter(Boolean);

const singlesEps = discographyItems
  .filter(item => ['Single', 'EP'].includes(getType(item)))
  .map(item => getBrowseId(item))
  .filter(Boolean);

console.log('Albums:', albums);
console.log('Singles/EPs:', singlesEps.length);
