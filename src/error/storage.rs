use std::fmt::Display;

use uuid::Uuid;

/// This enum gets used inside the Storage variant in the YtuwuError enum
/// Holds multiple different types of things that can go wrong in the downlaoder storage for
/// downloaded media and media bundles.
#[derive(Debug, Clone)]
pub enum StorageError {
    /// Extraction of the channel template with the given id failed.
    /// Holds the Uuid that was used to extract the template.
    ChannelTemplateExtraction(Uuid),
    /// Extraction of the playlist name with the given id failed.
    /// Holds the Uuid that was used to extract the name.
    ListNameExtraction(Uuid),
    /// Extaction of the media with the given id failed.
    /// Holds the Uuid that was used to extract the media.
    MediaExtraction(Uuid),
}

impl Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ChannelTemplateExtraction(id) => write!(f, "Channel template with the given id was not found in the channel template collection. Given id: {id}"),
            Self::ListNameExtraction(id) => write!(f, "Playlist name with the given id was not found in the playlist name collection. Given id: {id}"),
            Self::MediaExtraction(id) => write!(f, "Media with the given id was not found in the storage. Given id: {id}"),
        }
    }
}
