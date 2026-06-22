/// This metadata is contained in every downloaded media and contains publically exposed data that
/// give general information about the media.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaMetadata {
    pub title: String,
    pub author: String,
}

/// This metadata is contained in every downloaded playlist and contains publically exposed data that
/// give general information about the playlist.
#[derive(Debug)]
pub struct PlaylistMetadata {
    pub title: String,
    // TODO: maybe actually add the author
    //pub author: String,
    pub song_count: u16,
}

/// This metadata is contained in every downloaded channel and contains publically exposed data that
/// give general information about the channel.
#[derive(Debug)]
pub struct ChannelMetadata {
    pub name: String,
}

impl MediaMetadata {
    /// takes ownership of the title and the autor and returns a new instance of MediaMetadata
    pub fn new(title: &str, author: &str) -> Self {
        Self {
            title: title.to_owned(),
            author: author.to_owned(),
        }
    }
}

impl PlaylistMetadata {
    /// Takes ownership of the title and creates a new instance of PlaylistMetadata
    pub fn new(title: &str, song_count: u16) -> Self {
        Self { title: title.to_owned(), song_count }
    }
}

impl ChannelMetadata {
    /// Takes ownership of the title and creates a new instance of ChannelMetadata
    pub fn new(name: &str) -> Self {
        Self { name: name.to_owned() }
    }
}
