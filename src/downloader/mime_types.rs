#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MimeType {
    MP4,
    M4A,
    Webm,
}

impl MimeType {
    pub fn as_str(&self) -> &str {
        match &self {
            Self::MP4 => "mp4",
            Self::M4A => "m4a",
            Self::Webm => "webm",
        }
    }
}
