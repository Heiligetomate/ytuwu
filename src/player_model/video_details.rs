use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct VideoDetails {
    pub title: String,
    pub author: String,
    pub thumbnail: Thumbnails,
}

#[derive(Deserialize, Debug)]
pub struct Thumbnails {
    thumbnails: Vec<Thumbnail>,
}

#[derive(Deserialize, Debug)]
pub struct Thumbnail {
    url: String,
    width: u16,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ThumbnailResolution {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl ThumbnailResolution {
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

impl Thumbnails {
    pub fn url_by_resolution(&self, resolution: &ThumbnailResolution) -> Option<&str> {
        for thumbnail in self.thumbnails.iter() {
            if let Some(thumbnail_resolution) = ThumbnailResolution::from_width(thumbnail.width) {
                if thumbnail_resolution == *resolution {
                    return Some(&thumbnail.url);
                }
            } else {
                return None;
            }
        }
        None
    }
}
