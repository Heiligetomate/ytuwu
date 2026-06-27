use std::fmt::Display;

// TODO: put thumbnail resolutions as comments
// TODO: Use this as itag maybe
/// This enum defines the different resolutions for the thumbnails
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ThumbRes {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl ThumbRes {
    /// matches the given width and returns the correct thumbnail resolution as a variant of self
    /// Returns None if there was no matching resolution with that width.
    pub fn from_width(width: u16) -> Option<Self> {
        match width {
            120 => Some(Self::Low),
            320 => Some(Self::Medium),
            480 => Some(Self::High),
            640 => Some(Self::VeryHigh),
            _ => None,
        }
    }
}

impl Display for ThumbRes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw = match self {
            ThumbRes::Low => "Low",
            ThumbRes::Medium => "Medium",
            ThumbRes::High => "High",
            ThumbRes::VeryHigh => "VeryHigh",
        };
        write!(f, "Thumbnail resolution: {}", raw)
    }
}
