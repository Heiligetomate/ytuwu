use bytes::{Bytes, BytesMut};

use crate::{
    Result,
    downloader::{
        mime_types::MimeType,
        streams::{audio::AudioStream, core::MediaStream, long_video::LongVideoStream, muxed::MuxedStream, short_video::ShortVideoStream},
    },
    streams::Thumbnail,
};

#[derive(Debug, PartialEq, Eq)]
pub enum AnyStream {
    Audio(AudioStream),
    LongVideo(LongVideoStream),
    ShortVideo(ShortVideoStream),
    Muxed(MuxedStream),
    Thumbnail(Thumbnail),
}

impl AnyStream {
    pub fn into_dyn(&self) -> Box<&dyn MediaStream> {
        match self {
            Self::Audio(s) => Box::new(s),
            Self::LongVideo(s) => Box::new(s),
            Self::ShortVideo(s) => Box::new(s),
            Self::Muxed(s) => Box::new(s),
            Self::Thumbnail(s) => Box::new(s),
        }
    }
}

impl MediaStream for AnyStream {
    fn save(&self, path: &std::path::Path, file_name: &str) -> Result<()> {
        match self {
            Self::Audio(s) => s.save(path, file_name),
            Self::LongVideo(s) => s.save(path, file_name),
            Self::ShortVideo(s) => s.save(path, file_name),
            Self::Muxed(s) => s.save(path, file_name),
            Self::Thumbnail(s) => s.save(path, file_name),
        }
    }

    fn get_data(&self) -> &BytesMut {
        match self {
            Self::Audio(s) => s.get_data(),
            Self::LongVideo(s) => s.get_data(),
            Self::ShortVideo(s) => s.get_data(),
            Self::Muxed(s) => s.get_data(),
            Self::Thumbnail(s) => s.get_data(),
        }
    }

    fn push_data(&mut self, data: Bytes) {
        match self {
            Self::Audio(s) => s.push_data(data),
            Self::LongVideo(s) => s.push_data(data),
            Self::ShortVideo(s) => s.push_data(data),
            Self::Muxed(s) => s.push_data(data),
            Self::Thumbnail(s) => s.push_data(data),
        }
    }

    fn get_mime_type(&self) -> MimeType {
        match self {
            Self::Audio(s) => s.get_mime_type(),
            Self::LongVideo(s) => s.get_mime_type(),
            Self::ShortVideo(s) => s.get_mime_type(),
            Self::Muxed(s) => s.get_mime_type(),
            Self::Thumbnail(s) => s.get_mime_type(),
        }
    }
}
