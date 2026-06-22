use crate::{
    Result,
    error::YtuwuError,
    itags::Itag,
    models::{Stream, ThumbnailStream},
};

/// This struct is used for storing the extracted downloadable streams of a player response
/// Holds a vec of Stream which all contain the stream url and an itag for identification
#[derive(Debug)]
pub struct ExtractedStreams {
    streams: Vec<Stream>,
}

/// This struct is used for storing the extracted downloadable thumbnail streams of a player response
/// Holds a vec of thumbnail stream which all contain the stream url and the width for identification
#[derive(Debug)]
pub struct ExtractedThumbnails {
    streams: Vec<ThumbnailStream>,
}

impl ExtractedThumbnails {
    /// Creates a new instance of itself with the given streams
    pub fn new(streams: Vec<ThumbnailStream>) -> Self {
        Self { streams }
    }

    /// Takes the resolution as ThumbRes and tries to get the url stream for that by going throgh
    /// all streams and looking at the width
    /// Returns Err if there was no match on the passed resolution
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
    /// Creates a new instance of itself with the given streams
    pub fn new(streams: Vec<Stream>) -> Self {
        Self { streams }
    }

    // TODO: why is this returning option but the funciton above a result
    /// Takes the itag and tries to get the url stream for that itag by going throgh all streams and
    /// checking if the itag int matches the stream itag int
    /// Returns None if there was no matchign stream found
    pub fn get_url_by_itag(&self, itag: &impl Itag) -> Option<&str> {
        for stream in self.streams.iter() {
            if stream.itag == itag.to_int() {
                return Some(&stream.url);
            }
        }
        None
    }

    /// Gets the best stream for that itag "category".
    /// This will only trigger if the itag has the variant HIGHEST
    /// If the variant is not highest, it will try to get the resolution for that exact Itag
    /// If the Itag is hightes, it will try to get the hightest stream and will continue if that
    /// failed until it finds a valid stream.
    /// Returns Err if the stream for the specific itag does not exist or the full itag order was
    /// not existent. An example of this szenario is that there is a videoitag even though there are
    /// just streams in short format available.
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

// TODO: put thumbnail resolutions as comments
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
