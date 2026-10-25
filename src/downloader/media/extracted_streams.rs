use crate::{
    Result,
    error::YtuwuError,
    itags::{Itag, ThumbRes},
    models::{Stream, ThumbnailStream},
};

/// This struct is used for storing the extracted downloadable streams of a player response.
/// Holds a vec of Stream which all contain the stream url and an itag for identification.
#[derive(Debug)]
pub struct ExtractedStreams {
    streams: Vec<Stream>,
}

/// This struct is used for storing the extracted downloadable thumbnail streams of a player response.
/// Holds a vec of thumbnail stream which all contain the stream url and the width for identification.
#[derive(Debug)]
pub struct ExtractedThumbnails {
    streams: Vec<ThumbnailStream>,
}

impl ExtractedThumbnails {
    /// Creates a new instance of itself with the given streams.
    pub fn new(streams: Vec<ThumbnailStream>) -> Self {
        Self { streams }
    }

    /// Takes the resolution as ThumbRes and returns the url of the thumbnail whose width is the
    /// closest to it. `ThumbRes::Highest` returns the widest thumbnail.
    /// YouTube does not return the same set of widths for every client, so an exact width match
    /// would fail whenever the sizes differ from the expected ones.
    /// Returns Err if there are no thumbnails at all.
    pub fn get_thumbnail_url_by_res(&self, resolution: &ThumbRes) -> Result<&str> {
        let best = match resolution.target_width() {
            // takes the best
            None => self
                .streams
                .iter()
                .max_by_key(|t| t.width),
            // takes the one thats the closest
            Some(target) => self
                .streams
                .iter()
                .min_by_key(|t| t.width.abs_diff(target)),
        };
        best.map(|t| t.url.as_str())
            .ok_or_else(|| YtuwuError::NoMatchingThumbnail(resolution.to_string()))
    }
}

impl ExtractedStreams {
    /// Creates a new instance of itself with the given streams.
    pub fn new(streams: Vec<Stream>) -> Self {
        Self { streams }
    }

    /// Takes the itag and tries to get the url stream for that itag by going throgh all streams and
    /// checking if the itag int matches the stream itag int.
    /// Returns None if there was no matchign stream found.
    pub fn get_url_by_itag(&self, itag: &impl Itag) -> Result<&str> {
        for stream in self.streams.iter() {
            if stream.itag == itag.to_int() {
                return Ok(&stream.url);
            }
        }
        Err(YtuwuError::NoMatchingStream(itag.to_string()))
    }

    /// Gets the best stream for that itag "category".
    /// This will only trigger if the itag has the variant HIGHEST.
    /// If the variant is not highest, it will try to get the resolution for that exact Itag.
    /// If the Itag is hightes, it will try to get the hightest stream and will continue if that
    /// failed until it finds a valid stream.
    /// Returns Err if the stream for the specific itag does not exist or the full itag order was
    /// not existent. An example of this szenario is that there is a videoitag even though there are
    /// just streams in short format available.
    pub fn get_best_stream<I: Itag>(&self, itag: &I) -> Result<&str> {
        if !itag.is_highest() {
            return self.get_url_by_itag(itag);
        }

        let mut current_itag = *itag;
        let mut url: Result<&str> = self.get_url_by_itag(&current_itag);
        while url.is_err() {
            current_itag = current_itag.next_best()?;
            url = self.get_url_by_itag(&current_itag);
        }
        url
    }
}
