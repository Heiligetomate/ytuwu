use std::fs;

use bytes::Bytes;

use crate::{
    downloader::media_stream::{AudioStream, LongVideoStream, MediaStream, MuxedStream, ShortVideoStream},
    itag::{LongVideoItag, MuxedItag, ShortVideoItag},
};

#[test]
fn test_save_audio_stream() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path();
    let bytes = Bytes::from("meow");
    let expected_path = path.join("audio_stream_opus.webm");

    let mut audio_stream = AudioStream::new(crate::itag::AudioItag::OpusMedium);
    audio_stream.push_data(bytes);
    audio_stream
        .save(path, "audio_stream_opus")
        .unwrap();
    assert!(expected_path.exists());
    assert_eq!(
        fs::read_to_string(expected_path)
            .unwrap()
            .as_str(),
        "meow"
    );
}

#[test]
fn test_save_long_video_stream() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path();
    let bytes = Bytes::from("meow");
    let expected_path = path.join("long_video_stream.webm");
    let mut stream = LongVideoStream::new(LongVideoItag::WebM1080p);
    stream.push_data(bytes);
    stream
        .save(path, "long_video_stream")
        .unwrap();
    assert!(expected_path.exists());
    assert_eq!(
        fs::read_to_string(expected_path)
            .unwrap()
            .as_str(),
        "meow"
    );
}

#[test]
fn test_save_short_video_stream() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path();
    let bytes = Bytes::from("meow");
    let expected_path = path.join("short_video_stream.mp4");
    let mut stream = ShortVideoStream::new(ShortVideoItag::High);
    stream.push_data(bytes);
    stream
        .save(path, "short_video_stream")
        .unwrap();
    assert!(expected_path.exists());
    assert_eq!(
        fs::read_to_string(expected_path)
            .unwrap()
            .as_str(),
        "meow"
    );
}

#[test]
fn test_save_muxed_stream() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path();
    let bytes = Bytes::from("meow");
    let expected_path = path.join("muxed_stream.mp4");
    let mut stream = MuxedStream::new(MuxedItag::MuxedMP4);
    stream.push_data(bytes);
    stream
        .save(path, "muxed_stream")
        .unwrap();
    assert!(expected_path.exists());
    assert_eq!(
        fs::read_to_string(expected_path)
            .unwrap()
            .as_str(),
        "meow"
    );
}
