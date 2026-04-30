#!/bin/bash

echo ""
echo "release ids"

jq -r '.contents.singleColumnBrowseResultsRenderer.tabs[0].tabRenderer.content.sectionListRenderer.contents[] | select(.shelfRenderer.title.runs[0].text == "Albums & Singles") | .shelfRenderer.content.verticalListRenderer.items[] | .compactPlaylistRenderer.playlistId' artist.json

echo "" 
echo "video ids"

jq -r '.. | .videoId? // empty' artist.json | sort -u

