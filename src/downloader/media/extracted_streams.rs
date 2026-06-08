use crate::{
    Result,
    error::YtuwuError,
    itags::Itag,
    models::player::{Stream, ThumbnailStream},
};

#[derive(Debug)]
pub struct ExtractedStreams {
    streams: Vec<Stream>,
}

#[derive(Debug)]
pub struct ExtractedThumbnails {
    streams: Vec<ThumbnailStream>,
}

impl ExtractedThumbnails {
    pub fn new(streams: Vec<ThumbnailStream>) -> Self {
        Self { streams }
    }

    pub fn get_thumbnail_url_by_res(&self, resolution: &ThumbRes) -> Result<&str> {
        for thumbnail in self.streams.iter() {
            if let Some(thumbnail_resolution) = ThumbRes::from_width(thumbnail.width) {
                if thumbnail_resolution == *resolution {
                    return Ok(&thumbnail.url);
                }
            } else {
                continue;
            }
        }
        Err(YtuwuError::NoMatchingThumbnail)
    }
}

impl ExtractedStreams {
    pub fn new(streams: Vec<Stream>) -> Self {
        Self { streams }
    }

    pub fn get_url_by_itag(&self, itag: &impl Itag) -> Option<&str> {
        for stream in self.streams.iter() {
            if stream.itag == itag.to_int() {
                return Some(&stream.url);
            }
        }
        None
    }

    pub fn get_best_stream<I: Itag>(&self, itag: &I) -> Result<&str> {
        if !itag.is_highest() {
            return self
                .get_url_by_itag(itag)
                .ok_or(YtuwuError::NoMatchingStream);
        }

        let mut current_itag = *itag;
        let mut url: Option<&str> = self.get_url_by_itag(&current_itag);
        while url.is_none() {
            current_itag = current_itag.next_best()?;
            url = self.get_url_by_itag(&current_itag);
        }
        Ok(url.ok_or(YtuwuError::NoMatchingStream)?)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ThumbRes {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl ThumbRes {
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
