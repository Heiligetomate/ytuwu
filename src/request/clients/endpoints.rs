/// This endpoint is for getting the correct streams for downloading media
pub(super) const PLAYER_ENDPOINT: &str = "https://music.youtube.com/youtubei/v1/player?prettyPrint=false";

/// This endpoint is for resolving channel names such as @ntomusic to an actual id
pub(super) const RESOLVE_CHANNEL_NAME_ENDPOINT: &str = "https://music.youtube.com/youtubei/v1/navigation/resolve_url?prettyPrint=false";

/// This endpoint is used for every client other than the player and channel name resolver
pub(super) const BROWSE_ENDPOINT: &str = "https://music.youtube.com/youtubei/v1/browse?prettyPrint=false";
