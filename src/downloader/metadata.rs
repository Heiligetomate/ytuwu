#[derive(Debug)]
pub struct MediaMetadata {
    pub title: String,
    pub author: String,
    pub album: Option<String>,
}

#[derive(Debug)]
pub struct PlaylistMetadata {
    pub title: String, 
    pub author: String, 
}

impl MediaMetadata {
    pub fn new(title: &str, author: &str, album: Option<&str>) -> Self {
        Self {
            title: title.to_owned(),
            author: author.to_owned(),
            album: album.map(|s| s.to_owned())
        }
    }
}

impl PlaylistMetadata {
    pub fn new(title: &str, author: &str) -> Self {
        Self {
            title: title.to_owned(), 
            author: author.to_owned(),
        }
    }
}
