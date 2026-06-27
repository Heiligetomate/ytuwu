use std::fmt::{Debug, Display};

use crate::{
    Result,
    downloader::{mime_types::MimeType, streams::MediaStream},
    itags::AnyItag,
};

/// Itags are an identifier for the different downloadable streams.
/// YouTube always sends the stream with the corresponding Itag together.
/// Audio and Video streams are normally downloaded separately which.
/// There are other Itags too which means that we need to have different Itag structs.
/// This trait is needed to unite these different itags and allow generic functions for different
/// itags and different produced streams.
pub trait Itag: Copy + Debug + Send + Display {
    /// This holds the Media stream that is produced when downloading the stream that is
    /// corresponding to this itag.
    type Stream: MediaStream;
    /// This returns true if the itag is the highest in the itag order.
    /// This returns false if the itag is not the highest in the itag order.
    fn is_highest(&self) -> bool;
    /// Returns the highest itag in the itag order list
    fn highest() -> Self
    where
        Self: Sized;
    /// Consumes self and returns the next itag in the itag order
    /// Returns an Err if the itag is already the lowest.
    fn next_best(self) -> Result<Self>
    where
        Self: Sized;
    /// Returns the corresponding itag code for this itag.
    fn to_int(&self) -> u16;
    /// Returns the correct mime type for the itag.
    /// This is needed for the saving of the streams.
    fn get_mime_type(&self) -> MimeType;
    /// Creates a new MediaStream that is the correct one for this itag.
    /// The stream is a completely empty stream that needs to be stacked with the data.
    fn new_stream(self) -> Self::Stream;
    /// Converts the Itag to the AnyItag enum.
    /// This is needed to avoid using dynamic traits.
    fn to_any(self) -> AnyItag;
}
