use crate::downloader::itags::{audio::AudioItag, muxed::MuxedItag, short::ShortItag, video::VideoItag};

#[derive(Debug, Clone, Copy)]
pub enum AnyItag {
    Audio(AudioItag),
    Video(VideoItag),
    Short(ShortItag),
    Muxed(MuxedItag),
}
