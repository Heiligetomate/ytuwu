#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaMetadata {
    pub title: String,
    pub author: String,
}

#[derive(Debug)]
pub struct PlaylistMetadata {
    pub title: String,
    //pub author: String,
    pub song_count: u16,
}

#[derive(Debug)]
pub struct ChannelMetadata {
    pub name: String,
}

impl MediaMetadata {
    pub fn new(title: &str, author: &str) -> Self {
        Self {
            title: title.to_owned(),
            author: author.to_owned(),
        }
    }
}

impl PlaylistMetadata {
    pub fn new(title: &str, song_count: u16) -> Self {
        Self { title: title.to_owned(), song_count }
    }
}

impl ChannelMetadata {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_owned() }
    }
}
