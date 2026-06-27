use std::fmt::Display;

use crate::{
    downloader::{
        itags::{audio::AudioItag, muxed::MuxedItag, short::ShortItag, video::VideoItag},
        streams::AnyStream,
    },
    itags::{Itag, ThumbRes},
};

/// This enum holds a variant of any Itag.
/// Its used to prevent dynamic traits
/// It implements Itag by matching self and calling the function on the matched value
/// highest cant be called because it doesnt know which itag is present
#[derive(Debug, Clone, Copy)]
pub enum AnyItag {
    Audio(AudioItag),
    Video(VideoItag),
    Short(ShortItag),
    Muxed(MuxedItag),
    Thumbnail(ThumbRes),
}

impl Itag for AnyItag {
    type Stream = AnyStream;

    fn to_int(&self) -> u16 {
        match &self {
            Self::Video(i) => i.to_int(),
            Self::Audio(i) => i.to_int(),
            Self::Short(i) => i.to_int(),
            Self::Muxed(i) => i.to_int(),
            Self::Thumbnail(i) => i.to_int(),
        }
    }

    fn is_highest(&self) -> bool {
        match &self {
            Self::Video(i) => i.is_highest(),
            Self::Audio(i) => i.is_highest(),
            Self::Short(i) => i.is_highest(),
            Self::Muxed(i) => i.is_highest(),
            Self::Thumbnail(i) => i.is_highest(),
        }
    }

    fn next_best(self) -> crate::Result<Self>
    where
        Self: Sized,
    {
        match &self {
            Self::Video(i) => i.next_best().map(|i| Self::Video(i)),
            Self::Audio(i) => i.next_best().map(|i| Self::Audio(i)),
            Self::Short(i) => i.next_best().map(|i| Self::Short(i)),
            Self::Muxed(i) => i.next_best().map(|i| Self::Muxed(i)),
            Self::Thumbnail(i) => i
                .next_best()
                .map(|i| Self::Thumbnail(i)),
        }
    }

    fn new_stream(self) -> Self::Stream {
        match &self {
            Self::Video(i) => AnyStream::Video(i.new_stream()),
            Self::Audio(i) => AnyStream::Audio(i.new_stream()),
            Self::Short(i) => AnyStream::Short(i.new_stream()),
            Self::Muxed(i) => AnyStream::Muxed(i.new_stream()),
            Self::Thumbnail(i) => AnyStream::Thumbnail(i.new_stream()),
        }
    }

    fn get_mime_type(&self) -> crate::downloader::mime_types::MimeType {
        match &self {
            Self::Video(i) => i.get_mime_type(),
            Self::Audio(i) => i.get_mime_type(),
            Self::Short(i) => i.get_mime_type(),
            Self::Muxed(i) => i.get_mime_type(),
            Self::Thumbnail(i) => i.get_mime_type(),
        }
    }

    fn highest() -> Self
    where
        Self: Sized,
    {
        panic!("cant get highest itag of anyitag because the type is not known")
    }

    fn to_any(self) -> AnyItag {
        self
    }
}

impl Display for AnyItag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw = match self {
            AnyItag::Audio(audio_itag) => audio_itag.to_string(),
            AnyItag::Video(video_itag) => video_itag.to_string(),
            AnyItag::Short(short_itag) => short_itag.to_string(),
            AnyItag::Muxed(muxed_itag) => muxed_itag.to_string(),
            AnyItag::Thumbnail(thumb_res) => thumb_res.to_string(),
        };
        write!(f, "Any: {}", raw)
    }
}
