/// This struct is holds different mime type endings
/// This is important for saving mediastreams
/// Every itag implements .get_mime_type which returns the correct mime type for the expected
/// stream.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MimeType {
    /// .mp4 for videos, audio and muxed steams
    MP4,
    /// .m4a pure audio
    M4A,
    /// .webm For videos, audio and muxed streams
    Webm,
    /// .png for thumbnails or images in general
    Png,
}

impl MimeType {
    /// Returns the files ending as &str. This can then be added to the file name
    pub fn as_str(&self) -> &str {
        match &self {
            Self::MP4 => "mp4",
            Self::M4A => "m4a",
            Self::Webm => "webm",
            Self::Png => "png",
        }
    }
}
