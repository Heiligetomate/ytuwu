use std::{fmt::Debug, path::Path};

use bytes::{Bytes, BytesMut};

use crate::{Result, downloader::mime_types::MimeType};

/// This trait represents the downloaded media streams
/// This exists because the itag trait has a type field for what the downloaded results will be
pub trait MediaStream: Debug + Send {
    /// Returns a MimeType variant
    fn get_mime_type(&self) -> MimeType;
    /// saves the media stream to the path
    /// The files name will be the file name with the correct mime type
    fn save(&self, path: &Path, file_name: &str) -> Result<()>;
    /// Returns a reference to the data stored inside the mediastream  
    fn get_data(&self) -> &BytesMut;
    /// Adds a chunk of data to itself
    /// Needed for chunked download
    fn push_data(&mut self, data: Bytes);
}
