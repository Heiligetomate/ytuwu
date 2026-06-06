use crate::{
    downloader::streams::{audio::AudioStream, core::MediaStream, long_video::LongVideoStream, muxed::MuxedStream, short_video::ShortVideoStream},
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
