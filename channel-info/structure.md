contents.singleColumnBrowseResultsRenderer.tabs[0]
  .tabRenderer.content.sectionListRenderer.contents[9]
    .shelfRenderer.content.verticalListRenderer.items[*]
      .compactPlaylistRenderer.playlistId

artist.json
│
├── header
│   └── c4TabbedHeaderRenderer
│       ├── channelId = "UCGbyt2Lm2WlWULsMuhMeZvg"
│       ├── title = "NTO"
│       └── ...
│
└── contents
    └── singleColumnBrowseResultsRenderer
        └── tabs (array of 4 tabs)
            │
            ├── tab 0: "Home" (selected)
            │   └── content
            │       └── sectionListRenderer
            │           └── contents (array of shelves)
            │               │
            │               ├── shelf 0: "NTO 🔗 Sofiane Pamart | FOREVER FRIENDS"
            │               │   └── verticalListRenderer.items[]
            │               │       └── each is compactVideoRenderer (with videoId)
            │               │
            │               ├── shelf 1: "NTO - [  ] (EP)"
            │               │   └── verticalListRenderer.items[] (videoId)
            │               │
            │               ├── shelf 2: "NTO - Official Videos"
            │               │   └── videoId
            │               │
            │               ├── shelf 3: "NTO - Official Tracks & Remixes"
            │               ├── shelf 4: "NTO - Apnea"
            │               ├── shelf 5: "Remixes"
            │               ├── shelf 6: "Live"
            │               ├── shelf 7: "Shorts"
            │               ├── shelf 8: "Music videos"
            │               │
            │               └── shelf 9: "Albums & Singles"   ← HERE ARE YOUR RELEASE IDs
            │                   └── verticalListRenderer.items[]
            │                       └── each item is compactPlaylistRenderer
            │                           └── playlistId = "OLAK5uy_..."
            │
            ├── tab 1: "Videos"
            │   └── (no album IDs, only videoIds)
            │
            ├── tab 2: "Playlists"
            │   └── (user-created playlists, not album IDs)
            │
            └── tab 3: "About"
